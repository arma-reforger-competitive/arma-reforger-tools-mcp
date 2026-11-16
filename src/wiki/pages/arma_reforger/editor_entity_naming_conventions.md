---
title: Arma Reforger:Editor Entity Naming Conventions
url: https://community.bistudio.com/wiki/Arma_Reforger:Editor_Entity_Naming_Conventions
---

## Localisation String Naming

The string naming conventions in Arma Reforger regarding Editor Entities goes as follows:

```
AR-EditableEntity_Beehive_01_Small_Blue_Name
```

| Part | Followed by | Mandatory | Description |
| --- | --- | --- | --- |
| **AR-** | dash<br>`-` | Yes | Code for the name of the project (AR = Arma Reforger, A4 = Arma 4, etc), separated from the next variable by a hyphen `-`. |
| **EditableEntity_** | underscore<br>`_` | No | Always use `EditableEntity` for an in-game asset that is not a character, vehicle, weapon, or item of equipment. This variable should be followed by an underscore. |
| **Beehive_** | underscore<br>`_` | Yes | Noun, pertaining to the entity. If the noun is two words, such as "Beer Bottle", it must be arranged in PascalCase (e.g "BeerBottle") and *not* be connected with underscores (e.g "Beer_Bottle"). |
| **01_** | underscore<br>`_` | Yes | Number with a leading zero, allowing for the possibility of future variants. |
| **Small_** | underscore<br>`_` | No | Defines the asset's size. |
| **Blue_** | underscore<br>`_` | No | Defines the colour. |
| **Name** | n/a | Yes | Finally, all names for Editor Entities end with the label 'Name', indicating their function in-game. |

## Localisation Content Naming

The string naming conventions in Arma Reforger regarding Editor Entities goes as follows:

```
Small Cyan Table - Broken
```

All names should be in https://en.wikipedia.org/wiki/Title_case.
Also, ensure links to images are included in the String Editor so nomenclature can be checked by a native speaker.
This is essential, as many assets are misnamed on our initial pass.

| Part | Mandatory | Description |
| --- | --- | --- |
| **Small** | No | Size. The first component in the name relates to the asset's size, which may be either:<br>- Miniature<br>- Small<br>- Medium<br>- Large<br>- Massive<br>**Note:** "Miniature" should only be used when the asset is far smaller than it should be; as in a replica or model (e.g "Miniature Belfry").<br>Similarly, "Massive" should only be used for assets that are extraordinarily larger than normal. |
| **Cyan** | No | Colour This component relates to the asset's colour. It can be one of:<br>\| Colour \| Red \| Orange \| Yellow \| Green \| Cyan \| Blue \| Magenta \| Purple \| White \| Black \| Gray \| Silver \| Pink \| Maroon \| Brown \| Beige \| Tan \| Peach \| Lime \| Olive \| Turquoise \| Teal \| Navy blue \| Indigo \| Violet \|<br>\| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \| --- \|<br>\| **Sample** \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|  \|<br>If the asset contains a myriad of colours, the prefix 'multi-colored' can be used. Try to avoid colours that are also materials, as this can be confusing (e.g., 'Ivory Crate' or 'Gold Container').<br><br>When a colour combination is involved, it is acceptable to use the conjunction 'and' (e.g., Blue and Red). Precedence in such pairings should be alphabetical (e.g., Beige and Black, Tan and Teal, etc.). |
| **Table** | Yes | Base Descriptor. This part is the most essential and complex. It must consist of at least a noun, but will often be preceded by an adjective. It may also be split by a preposition or conjunction, although prepositions should be avoided if grammatically possible. Note; any adjective relating to the state of the asset (e.g., 'Old', 'New', 'Rusted', 'Broken', 'Open', 'Closed') should be separated as a hyphenated adjective (see below).<br>\| Correct \| Incorrect \|<br>\| --- \| --- \|<br>\| - Standing Lamp<br>- Brick Stack<br>- Mop and Bucket<br>- Flowers in Pot<br>- Wicker Basket with Firewood \| - Stack of Bricks (> Brick Stack)<br>- Pile of Scrap (> Scrap Pile)<br>- Mop in Bucket (> Mop and Bucket)<br>- Old Water Cistern (> Water Cistern - Old)<br>- Old Opened Barrel (> Barrel - Old, Open) \| |
| **Broken** | No | Adjectives. Finally, certain adjectives may be tacked to the end of the name *via* a hyphen.<br>This can be particularly useful for specification, where the noun of the base descriptor is not sufficient (e.g., 'Statue' > 'Statue - Soldier', 'Sign' > 'Sign - Slow Speed').<br>Common hyphenated adjectives can be taken from the list below.<br>Assets where a pair of adjectives are needed should be divided by a comma and a space (e.g., - Old, Open),<br>and should be ordered alphabetically *except* with non-permanent states, which should always be last (e.g., 'Open', 'Closed', 'Locked', 'Unlocked', 'On', 'Off', etc.).<br>\| Correct (permanent states) \| Incorrect (temporary states) \|<br>\| --- \| --- \|<br>\| - Old<br>- New<br>- Rusted<br>- Broken<br>- Dismantled<br>- Camouflaged \| - On<br>- Off<br>- Empty<br>- Full<br>- Open<br>- Closed \| |

## See Also

- Localisation plugin, accessible in [Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager) with [Ctrl+Alt+L]
