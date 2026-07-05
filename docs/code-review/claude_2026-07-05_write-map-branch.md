# Revue de code — branche `features/write-map`

> Revue multi-agent (effort élevé) sur le diff `master...HEAD`
> 24 agents · 4 finders (correctness × 3 angles + cleanup) · vérification indépendante par finding
> **10 findings validés** : 7 régressions de correctness + 3 nettoyages.

---

## 🔴 Correctness — régressions

### 1. `Map::save` panique sur erreur I/O au lieu de retourner `Err`
**`crates/wce_map/src/map.rs:198`** · CONFIRMÉ

`save()` retourne `Result<(), MapError>`, mais son helper `save_file()` fait `.unwrap()` sur
`File::create` et `write_all`. Un chemin en lecture seule, inexistant, ou un disque plein fait
paniquer tout le process au lieu de remonter une `MapError`. Contredit directement la priorité
« remplacer les `panic!` par `Result` » de CLAUDE.md.

### 2. Support des maps Reforged (w3i v28) supprimé
**`crates/wce_map/src/w3i_file.rs:632`** · CONFIRMÉ

L'arm `28 => Ok(Reforged)` de `to_game_version` est commenté → la version 28 tombe dans l'arm
d'erreur. Ouvrir une map Reforged échoue désormais avec `"Unknown or unsupported game version
'28'"` ; `Map::open` échoue complètement. Régression : ces maps se chargeaient avant.

### 3. Validation de version du `.imp` supprimée
**`crates/wce_map/src/import_file.rs:88`** · CONFIRMÉ

`to_game_version(reader.read_u32()?)` (qui rejetait toute valeur ≠ 0/1) a été remplacé par
`reader.skip(4)`. Un `war3map.imp` corrompu ou d'un format futur n'est plus détecté : les 4 octets
sont ignorés silencieusement et le parsing continue sur une liste d'imports potentiellement mal
interprétée.

### 4. Type de chemin d'import RoC perdu à la sauvegarde
**`crates/wce_map/src/import_file.rs:126`** · CONFIRMÉ

Pour les maps RoC, le reader écrase le vrai type-byte par `ImportPathType::RoC`, et `to_u8()`
réémet `0` à l'écriture. Un import RoC avec un type de chemin custom (10/13 selon la spec
`52_imp_imported_files.md`) est réécrit à 0 → WC3 ne résout plus l'asset (modèle/texture/son
manquant).

> **Nuance :** un vérificateur a montré que la fixture RoC réelle a le byte 0 et round-trip
> byte-exact. L'impact concret dépend de l'existence de maps RoC avec un type-byte non nul —
> à confirmer.

### 5. Enum `BlpFlag` et sa validation supprimés
**`crates/wce_formats/src/blp.rs:205`** · CONFIRMÉ

`BlpFlag::from` (qui renvoyait `BLPError::UnknownFlag` pour les valeurs hors plage) a disparu ;
le champ flags est stocké brut sans validation. Un header BLP malformé/non supporté est désormais
accepté silencieusement au lieu de remonter une erreur.

### 6. Flag `beginParameters` du trigger réécrit à tort
**`crates/wce_map/src/triggers/trigger_data.rs:262`** · PLAUSIBLE

`SubParameters::write` dérive le flag de `parameters.len() > 0`, mais le reader peut aboutir à
`parameters` vide alors que le flag on-disk valait 1. Au round-trip le `war3map.wtg` diffère de
l'original de 4 octets, ce qui peut désynchroniser l'attente du World Editor pour cet appel de
fonction.

### 7. Assertion EOF post-parse du BLP supprimée
**`crates/wce_formats/src/blp.rs:240`** · PLAUSIBLE

L'`assert_eq!(reader.size(), reader.pos(), ...)` a été retiré. Un BLP dont la table
d'offsets/tailles de mipmaps ne couvre pas tout le buffer parse maintenant « avec succès » ;
comme le writer re-sérialise depuis les fragments bruts stockés, le minimap round-trippé peut
être silencieusement tronqué/corrompu au lieu d'échouer au parse.

---

## 🟡 Cleanup

### 8. Mipmaps JPEG décodés mais jamais utilisés
**`crates/wce_formats/src/blp.rs:121`** · CONFIRMÉ

`parse_jpeg_mipmaps` décode entièrement chaque mipmap en `Vec<RGB8>` (`jpeg_mipmaps`) que personne
ne lit (getter commenté ; seul `jpeg_mipmaps.len()` est consommé, déjà fourni par
`jpeg_mipmaps_raw.len()`). Le write utilise `jpeg_mipmaps_raw`. → CPU/mémoire gaspillés à chaque
ouverture de map.

### 9. ~80 lignes dupliquées à l'identique dans 6 fichiers `custom_datas`
**`crates/wce_map/src/custom_datas/ability.rs:99`** · CONFIRMÉ

`prepare_write`/`write`/`read_opt`/`read_file` sont copiés quasi mot pour mot dans ability, buff,
destructable, doodad, item, upgrade — ne diffèrent que par `FILE_NAME` et le type d'erreur. Tout
correctif (ex. règle « skip si les deux listes sont vides ») doit être appliqué 6 fois et dérivera
silencieusement si on en oublie un. Un générique sur le type d'objet ferait le travail.

### 10. `to_write_error` calcule `_pos`/`_size` puis les jette
**`crates/wce_formats/src/binary_writer.rs:234`** · CONFIRMÉ

Le helper récupère position/taille du buffer (laissant croire qu'il enrichit l'erreur) mais les
jette et renvoie juste `WriteError::IoError(error)`. Code mort qui induit en erreur : aucun
contexte de position n'est préservé.

---

## Écartés par la vérification (3)

- **`import_file.rs:94`** — la fixture RoC réelle round-trip byte-exact (0→0).
- **`custom_datas/mod.rs:158`** — `for i in 0..len { .get(i).unwrap() }` ne peut jamais paniquer,
  pur style.
- **`w3i_file.rs:641`** — `Reforged => unimplemented!()` sur le write path n'est pas atteignable
  car `version` n'est jamais mis à Reforged (conséquence du finding #2, pas un bug distinct).

---

## Fil rouge

Plusieurs findings viennent de validations retirées pendant l'ajout du write path (#2, #3, #5, #7).
À vérifier : ces suppressions étaient-elles intentionnelles ou des dommages collatéraux du
refactoring ? Les #2 et #3 semblent des régressions non voulues.

**Priorité de correction recommandée :** #1 (panic I/O) et #2 (Reforged), les plus impactants.

---

## Résolution (2026-07-05)

| # | Finding | Statut |
|---|---------|--------|
| 1 | `Map::save` panique sur erreur I/O | ✅ Corrigé — `save_file` retourne `MapError::SaveFileIo` |
| 2 | Support Reforged (w3i v28) supprimé | ✅ Corrigé — arm `28 => Reforged` restauré ; write Reforged = erreur propre (couvre aussi le finding écarté `w3i_file.rs:641`, devenu atteignable) |
| 3 | Validation de version du `.imp` supprimée | ✅ Corrigé — version validée (≤ 1) et préservée au round-trip |
| 4 | Type de chemin d'import RoC perdu | ✅ Corrigé — `ImportPathType::RoC(u8)` porte l'octet on-disk |
| 5 | Enum `BlpFlag` supprimé | 🟦 Accepté — `Flags` est un bitfield combinable (`specs/blp.txt:29,49`) ; l'ancienne validation par égalité était incorrecte. Champ gardé brut par conception (`blp.rs`, commentaires du champ et de `has_alpha`) |
| 6 | Flag `beginParameters` réécrit à tort | ✅ Corrigé — flag on-disk stocké et réémis ; round-trip `.wtg` byte-exact testé sur les deux sandbox |
| 7 | Assertion EOF post-parse BLP supprimée | 🟦 Accepté — le spec autorise du padding (`specs/blp.txt:86-88`) ; un overrun est déjà attrapé par `read_bytes`. Suppression intentionnelle et commentée dans `BLP::from` |
| 8 | Mipmaps JPEG décodés jamais utilisés | ✅ Corrigé — décodage eager supprimé, différé à la phase 2 (todos/11 point 7) |
| 9 | ~80 lignes dupliquées dans `custom_datas` | ✅ Corrigé — 7 fichiers (les 6 cités + `unit.rs`, copie identique) dédupliqués via `CustomObjectsFile<K>` |
| 10 | `to_write_error` jette `_pos`/`_size` | ✅ Corrigé — helper supprimé, `map_err(WriteError::IoError)` direct |
