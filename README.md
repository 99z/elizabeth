# elizabeth ![](https://github.com/99z/elizabeth/workflows/Rust/badge.svg) [![license: WTFPL](https://img.shields.io/badge/license-WTFPL-brightgreen.svg)](http://www.wtfpl.net/about/)


```shell script
Usage: liz [-s <shadow>] -p <persona> [-a]

Find shadow resistance/weakness information

Options:
  -s, --shadow      name of shadow
  -p, --persona     persona series number. One of: 3, 4
  -a, --all         get all shadow resistance info for specified game. Defaults
                    to false
  --help            display usage information

```
```shell script
$ liz -p 3 -s 'magical magus'
Magical Magus

Sub-boss
STRONG: Slash Strike Pierce 
DRAIN: Ice
NEUTRAL: Elec Wind Almi 
WEAK: Fire 
NULL: Light Dark 

Normal enemy
NEUTRAL: Slash Strike Pierce Elec Wind Light Dark Almi 
WEAK: Fire 
NULL: Ice 

```

## todo
- [ ] test against all 3/4/5 shadow pages, including variants
- [ ] scrape tables for p3/4/5 and output complete shadow info to JSON
- [ ] Elizabeth's requests lookup
- [ ] P4 quests lookup

## contributing
just use it and open an issue with the shadow name if the info it spits out is bogus vs. the wiki.
I should test that myself somehow but man it's gonna be tedious

## rant
wikia's API is awful, man. Maybe it's not their fault, maybe the lack of standardization on the SMT wiki individually.

check this out, an example response from hitting https://megamitensei.fandom.com/api/v1/Articles/AsJson?id=10965:

```json
{"content":"<aside class=\"portable-infobox pi-background pi\"><div class=\"pi-item pi-data pi-item-spacing pi-border-color\" data-source=\"japan\">\n\t\n\t\t<h3 class=\"pi-data-label pi-secondary-font\">Japanese Name<\/h3>\n\t\n\t<div class=\"pi-data-value pi-font\"><span lang=\"ja\">泣くテーブル<\/span><\/div>\n<\/div>\n\n<div class=\"pi-item pi-data pi-item-spacing pi-border-color\" data-source=\"romaji\">\n\t\n\t\t<h3 class=\"pi-data-label pi-secondary-font\">Rōmaji<\/h3>\n\t\n\t<div class=\"pi-data-value pi-font\"><span lang=\"ja-Latn-hepburn\"><i>Naku Tēburu<\/i><\/span><\/div>\n<\/div>\n\n<div class=\"pi-item pi-data pi-item-spacing pi-border-color\" data-source=\"first appearance\">\n\t\n\t\t<h3 class=\"pi-data-label pi-secondary-font\">First Appearance<\/h3>\n\t\n\t<div class=\"pi-data-value pi-font\">Persona 3<\/div>\n<\/div>\n\n<div class=\"pi-item pi-data pi-item-spacing pi-border-color\" data-source=\"arcana\">\n\t\n\t\t<h3 class=\"pi-data-label pi-secondary-font\">Arcana(s)<\/h3>\n\t\n\t<div class=\"pi-data-value pi-font\"><a href=\"\/wiki\/Magician_Arcana\" title=\"Magician Arcana\">Magician<\/a><\/div>\n<\/div>\n\n<div class=\"pi-item pi-data pi-item-spacing pi-border-color\" data-source=\"species\">\n\t\n\t\t<h3 class=\"pi-data-label pi-secondary-font\">Species<\/h3>\n\t\n\t<div class=\"pi-data-value pi-font\"><a href=\"\/wiki\/Shadow_(Persona)\" title=\"Shadow (Persona)\">Shadow<\/a><\/div>\n<\/div>\n<\/aside> <p><\/p><p><b>Crying Table<\/b> is a <a href=\"\/wiki\/Shadow_(Persona)\" title=\"Shadow (Persona)\">Shadow<\/a> in the Persona series. <\/p> <h2 id=\"Appearances\" section=\"1\" aria-controls=\"Appearances-collapsible-section\"><div class=\"section-header-label\">Appearances<\/div><svg class=\"wds-icon wds-icon-small chevron\" viewbox=\"0 0 18 18\" width=\"18\" height=\"18\"><use xmlns:xlink=\"http:\/\/www.w3.org\/1999\/xlink\" xlink:href=\"#wds-icons-menu-control-small\"><\/use><\/svg><\/h2><section id=\"Appearances-collapsible-section\" aria-pressed=\"false\" aria-expanded=\"false\" class=\"mobile-hidden\"><ul><li><i><a href=\"\/wiki\/Persona_3\" title=\"Persona 3\">Persona 3<\/a><\/i> \/ <i><a href=\"\/wiki\/Persona_3_FES\" title=\"Persona 3 FES\">Persona 3 FES<\/a><\/i> \/ <i><a href=\"\/wiki\/Persona_3_Portable\" title=\"Persona 3 Portable\">Persona 3 Portable<\/a><\/i> <\/li><li><i><a href=\"\/wiki\/Persona_4\" title=\"Persona 4\">Persona 4<\/a><\/i> \/ <i><a href=\"\/wiki\/Persona_4_Golden\" title=\"Persona 4 Golden\">Persona 4 Golden<\/a><\/i> <\/li><\/ul><\/section><h2 id=\"Profile\" section=\"2\" aria-controls=\"Profile-collapsible-section\"><div class=\"section-header-label\">Profile<\/div><svg class=\"wds-icon wds-icon-small chevron\" viewbox=\"0 0 18 18\" width=\"18\" height=\"18\"><use xmlns:xlink=\"http:\/\/www.w3.org\/1999\/xlink\" xlink:href=\"#wds-icons-menu-control-small\"><\/use><\/svg><\/h2><section id=\"Profile-collapsible-section\" aria-pressed=\"false\" aria-expanded=\"false\" class=\"mobile-hidden\"><h3 id=\"Persona_3\" section=\"3\">Persona 3<\/h3> <p>Three Crying Table shadows act as a sub-boss found on the 25th floor of <a href=\"\/wiki\/Tartarus\" title=\"Tartarus\">Tartarus<\/a> in <a href=\"\/wiki\/Arqa_Block\" title=\"Arqa Block\">Arqa Block<\/a>. They impede the party's progress and must be beaten to proceed. The Crying Table later reappears as a regular enemy in the Arqa Block of Tartarus between floors 48 to 63, and as a boss on the 8th floor of <a href=\"\/wiki\/Ptolomea\" title=\"Ptolomea\">Ptolomea<\/a> with the <a href=\"\/wiki\/Wondrous_Magus\" title=\"Wondrous Magus\">Wondrous Magus<\/a> and <a href=\"\/wiki\/Cowardly_Maya\" title=\"Cowardly Maya\">Cowardly Maya<\/a>. <\/p> <h3 id=\"Persona_4\" section=\"4\">Persona 4<\/h3> <p>Crying Tables appear in the 9th and 10th Halls of the <a href=\"\/wiki\/Steamy_Bathhouse\" title=\"Steamy Bathhouse\">Steamy Bathhouse<\/a> in the <a href=\"\/wiki\/Midnight_Channel\" title=\"Midnight Channel\">Midnight Channel<\/a>. In battle, they will first set up with Marakunda and Matarukaja before attacking with Magaru. Upon being defeated, they may drop Brave Lumber. Selling 10 of these to Daidara will unlock the Bravery Vessel accesory in the shop, which will reduce a party member's chance of being afflicted with Fear. <\/p><p>The daughter of the model salesman (found on the second floor of the Practice Building) will ask you to find her a Reflecting Board for Quest #12: Desk Refurbishing, Part 2. After accepting her request, the Crying Table will drop a Reflecting Board upon being defeated. It will not drop Brave Lumber until the quest is completed. <\/p> <h3 id=\"Persona_4_Golden\" section=\"5\">Persona 4 Golden<\/h3> <p>It is found in Halls 8 to 10 of Steamy Bathhouse. It may drop Fluorite or Brave Lumber upon defeat. Selling 4 Brave Lumbers to Daidara will unlock the Bravery Vessel accesory in the shop. <\/p> <\/section><h2 id=\"Stats\" section=\"6\" aria-controls=\"Stats-collapsible-section\"><div class=\"section-header-label\">Stats<\/div><svg class=\"wds-icon wds-icon-small chevron\" viewbox=\"0 0 18 18\" width=\"18\" height=\"18\"><use xmlns:xlink=\"http:\/\/www.w3.org\/1999\/xlink\" xlink:href=\"#wds-icons-menu-control-small\"><\/use><\/svg><\/h2><section id=\"Stats-collapsible-section\" aria-pressed=\"false\" aria-expanded=\"false\" class=\"mobile-hidden\"><h3 id=\"Persona_3_2\" section=\"7\">Persona 3<\/h3> <div class=\"tabber\"><div class=\"tabbertab\" title=\"Sub-Boss\"><p> <\/p><table><tr><td> <table><tr><td> <table class=\"customtable\"><tr><th><a href=\"\/wiki\/Arcana\" title=\"Arcana\"><span>Arcana<\/span><\/a> <\/th><th>Level <\/th><th>HP <\/th><th>SP <\/th><td rowspan=\"2\"> <table><tr><td>Strength <\/td><td>13 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Magic <\/td><td>16 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Endurance <\/td><td>16 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Agility <\/td><td>11 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Luck <\/td><td>9 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><\/table><\/td><\/tr><tr><td><a href=\"\/wiki\/Magician_Arcana\" title=\"Magician Arcana\">Magician<\/a> <\/td><td>14 <\/td><td>260<div><\/div> <\/td><td>260<div><\/div> <\/td><\/tr><\/table><table class=\"customtable\"><tr><th>Slash <\/th><th>Strike <\/th><th>Pierce <\/th><th>Fire <\/th><th>Ice <\/th><th title=\"Electricity\">Elec <\/th><th>Wind <\/th><th>Light <\/th><th>Dark <\/th><th title=\"Almighty\">Almi <\/th><\/tr><tr><td>Strong <\/td><td>Null <\/td><td>Strong <\/td><td>Drain <\/td><td>Weak <\/td><td>- <\/td><td>- <\/td><td>Null <\/td><td>Null <\/td><td>- <\/td><\/tr><\/table><\/td><\/tr><\/table><table class=\"customtable\"><tr><th colspan=\"4\"><a href=\"\/wiki\/List_of_Persona_3_Skills\" title=\"List of Persona 3 Skills\"><span>List of Skills<\/span><\/a> <\/th><\/tr><tr><th>Skill <\/th><th>Effect <\/th><\/tr><tr><th>Strike Attack <\/th><td>Normal attack using the Strike attribute. <\/td><\/tr><tr><th>Fire Boost <\/th><td>Strengthens Fire attacks by 25%. <\/td><\/tr><tr><th>Maragi <\/th><td>Deals light Fire damage to all foes. <\/td><\/tr><tr><th>Agilao <\/th><td>Deals medium Fire damage to one foe. <\/td><\/tr><tr><th>Poisma <\/th><td>Poisons 1 foe. (25% chance) <\/td><\/tr><tr><th>Mighty Swing <\/th><td>Deals medium Slash damage to one foe. <\/td><\/tr><tr><th>Torrent Shot <\/th><td>Deals light Pierce damage to one foe. (2-3 hits) <\/td><\/tr><\/table><\/td><\/tr><\/table><\/div><div class=\"tabbertab\" title=\"The Journey\"><p> <\/p><table><tr><td> <table><tr><td> <table class=\"customtable\"><tr><th><a href=\"\/wiki\/Arcana\" title=\"Arcana\"><span>Arcana<\/span><\/a> <\/th><th>Level <\/th><th>HP <\/th><th>SP <\/th><td rowspan=\"2\"> <table><tr><td>Strength <\/td><td>15 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Magic <\/td><td>18 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Endurance <\/td><td>14 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Agility <\/td><td>13 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Luck <\/td><td>10 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><\/table><\/td><\/tr><tr><td><a href=\"\/wiki\/Magician_Arcana\" title=\"Magician Arcana\">Magician<\/a> <\/td><td>21 <\/td><td>170<div><\/div> <\/td><td>97<div><\/div> <\/td><\/tr><\/table><table class=\"customtable\"><tr><th>Slash <\/th><th>Strike <\/th><th>Pierce <\/th><th>Fire <\/th><th>Ice <\/th><th title=\"Electricity\">Elec <\/th><th>Wind <\/th><th>Light <\/th><th>Dark <\/th><th title=\"Almighty\">Almi <\/th><\/tr><tr><td>- <\/td><td>- <\/td><td>- <\/td><td>Null <\/td><td>Weak <\/td><td>- <\/td><td>- <\/td><td>- <\/td><td>- <\/td><td>- <\/td><\/tr><\/table><\/td><\/tr><\/table><table class=\"customtable\"><tr><th colspan=\"4\"><a href=\"\/wiki\/List_of_Persona_3_Skills\" title=\"List of Persona 3 Skills\"><span>List of Skills<\/span><\/a> <\/th><\/tr><tr><th>Skill <\/th><th>Effect <\/th><\/tr><tr><th>Strike Attack <\/th><td>Normal attack using the Strike attribute. <\/td><\/tr><tr><th>Maragi <\/th><td>Deals light Fire damage to all foes. <\/td><\/tr><tr><th>Dekaja <\/th><td>Nullifies stat bonuses on all foes. <\/td><\/tr><\/table><\/td><\/tr><\/table><\/div><div class=\"tabbertab\" title=\"The Answer\"><p> <\/p><table><tr><td> <table><tr><td> <table class=\"customtable\"><tr><th><a href=\"\/wiki\/Arcana\" title=\"Arcana\"><span>Arcana<\/span><\/a> <\/th><th>Level <\/th><th>HP <\/th><th>SP <\/th><td rowspan=\"2\"> <table><tr><td>Strength <\/td><td>35 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Magic <\/td><td>38 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Endurance <\/td><td>34 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Agility <\/td><td>33 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Luck <\/td><td>30 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><\/table><\/td><\/tr><tr><td><a href=\"\/wiki\/Magician_Arcana\" title=\"Magician Arcana\">Magician<\/a> <\/td><td>48 <\/td><td>1,300<div><\/div> <\/td><td>??<div><\/div> <\/td><\/tr><\/table><table class=\"customtable\"><tr><th>Slash <\/th><th>Strike <\/th><th>Pierce <\/th><th>Fire <\/th><th>Ice <\/th><th title=\"Electricity\">Elec <\/th><th>Wind <\/th><th>Light <\/th><th>Dark <\/th><th title=\"Almighty\">Almi <\/th><\/tr><tr><td>- <\/td><td>Drain <\/td><td>- <\/td><td>Drain <\/td><td>Weak <\/td><td>Repel <\/td><td>- <\/td><td>Null <\/td><td>Null <\/td><td>- <\/td><\/tr><\/table><\/td><\/tr><\/table><table class=\"customtable\"><tr><th colspan=\"4\"><a href=\"\/wiki\/List_of_Persona_3_Skills\" title=\"List of Persona 3 Skills\"><span>List of Skills<\/span><\/a> <\/th><\/tr><tr><th>Skill <\/th><th>Effect <\/th><\/tr><tr><th>Agidyne <\/th><td>Deals heavy Fire damage to one foe. <\/td><\/tr><tr><th>Maragion <\/th><td>Deals medium Fire damage to all foes. <\/td><\/tr><tr><th>Life Drain <\/th><td>Drains 35 HP from one foe. <\/td><\/tr><tr><th>Spirit Drain <\/th><td>Drains 20 SP from one foe. <\/td><\/tr><tr><th>Makarakarn <\/th><td>Barrier that reflects magic damage 1x per ally. <\/td><\/tr><tr><th>Evil Smile <\/th><td>Instills Fear in all foes. (25% chance) <\/td><\/tr><tr><th>Fire Amp <\/th><td>Greatly strengthens Fire attacks by 50%. <\/td><\/tr><tr><th>Dodge Ice <\/th><td>Evasion rate doubled vs Ice attacks. <\/td><\/tr><\/table><\/td><\/tr><\/table><\/div><\/div> <h3 id=\"Persona_4_2\" section=\"8\">Persona 4<\/h3> <div class=\"tabber\"><div class=\"tabbertab\" title=\"Persona 4\"><p> <\/p><table><tr><td> <table><tr><td> <table class=\"customtable\"><tr><th><a href=\"\/wiki\/Arcana\" title=\"Arcana\"><span>Arcana<\/span><\/a> <\/th><th>Level <\/th><th>HP <\/th><th>SP <\/th><td rowspan=\"2\"> <table><tr><td>Strength <\/td><td>12 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Magic <\/td><td>15 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Endurance <\/td><td>15 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Agility <\/td><td>16 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Luck <\/td><td>12 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><\/table><\/td><\/tr><tr><td><a href=\"\/wiki\/Magician_Arcana\" title=\"Magician Arcana\">Magician<\/a> <\/td><td>24 <\/td><td>139<div><\/div> <\/td><td>177<div><\/div> <\/td><\/tr><\/table><table class=\"customtable\"><tr><th title=\"Physical\">Phys <\/th><th>Fire <\/th><th>Ice <\/th><th title=\"Electricity\">Elec <\/th><th>Wind <\/th><th>Light <\/th><th>Dark <\/th><th title=\"Almighty\">Almi <\/th><\/tr><tr><td>Strong <\/td><td>Strong <\/td><td>Weak <\/td><td>Strong <\/td><td>Strong <\/td><td>- <\/td><td>- <\/td><td>- <\/td><\/tr><\/table><table class=\"customtable\"><tr><th>EXP <\/th><th>Yen <\/th><th><a href=\"\/wiki\/List_of_Persona_4_Items\" title=\"List of Persona 4 Items\"><span>Normal Drop<\/span><\/a> <\/th><th><a href=\"\/wiki\/List_of_Persona_4_Items\" title=\"List of Persona 4 Items\"><span>Rare Drop<\/span><\/a> <\/th><\/tr><tr><td>460 <\/td><td>250 <\/td><td>- <\/td><td>- <\/td><\/tr><\/table><\/td><\/tr><\/table><table class=\"customtable\"><tr><th colspan=\"4\"><a href=\"\/wiki\/List_of_Persona_4_Skills\" title=\"List of Persona 4 Skills\"><span>List of Skills<\/span><\/a> <\/th><\/tr><tr><th>Skill <\/th><th>Effect <\/th><\/tr><tr><th>Marakunda <\/th><td>Decreases all foes' Defense for 3 turns. <\/td><\/tr><tr><th>Matarukaja <\/th><td>Increases party's Attack for 3 turns. <\/td><\/tr><tr><th>Magaru <\/th><td>Deals light Wind damage to all foes. <\/td><\/tr><\/table><\/td><\/tr><\/table><\/div><div class=\"tabbertab\" title=\"Persona 4 Golden\"><p> <\/p><table><tr><td> <table><tr><td> <table class=\"customtable\"><tr><th><a href=\"\/wiki\/Arcana\" title=\"Arcana\"><span>Arcana<\/span><\/a> <\/th><th>Level <\/th><th>HP <\/th><th>SP <\/th><td rowspan=\"2\"> <table><tr><td>Strength <\/td><td>12 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Magic <\/td><td>15 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Endurance <\/td><td>15 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Agility <\/td><td>16 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><tr><td>Luck <\/td><td>12 <\/td><td><div><div><\/div><div><\/div><\/div> <\/td><\/tr><\/table><\/td><\/tr><tr><td><a href=\"\/wiki\/Magician_Arcana\" title=\"Magician Arcana\">Magician<\/a> <\/td><td>24 <\/td><td>139<div><\/div> <\/td><td>177<div><\/div> <\/td><\/tr><\/table><table class=\"customtable\"><tr><th title=\"Physical\">Phys <\/th><th>Fire <\/th><th>Ice <\/th><th title=\"Electricity\">Elec <\/th><th>Wind <\/th><th>Light <\/th><th>Dark <\/th><th title=\"Almighty\">Almi <\/th><\/tr><tr><td>Strong <\/td><td>Strong <\/td><td>Weak <\/td><td>Strong <\/td><td>Strong <\/td><td>- <\/td><td>- <\/td><td>- <\/td><\/tr><\/table><table class=\"customtable\"><tr><th>EXP <\/th><th>Yen <\/th><th><a href=\"\/wiki\/List_of_Persona_4_Items\" title=\"List of Persona 4 Items\"><span>Normal Drop<\/span><\/a> <\/th><th><a href=\"\/wiki\/List_of_Persona_4_Items\" title=\"List of Persona 4 Items\"><span>Rare Drop<\/span><\/a> <\/th><\/tr><tr><td>440 <\/td><td>220 <\/td><td>- <\/td><td>- <\/td><\/tr><\/table><\/td><\/tr><\/table><table class=\"customtable\"><tr><th colspan=\"4\"><a href=\"\/wiki\/List_of_Persona_4_Skills\" title=\"List of Persona 4 Skills\"><span>List of Skills<\/span><\/a> <\/th><\/tr><tr><th>Skill <\/th><th>Effect <\/th><\/tr><tr><th>Makajam <\/th><td>Silences 1 foe (40% chance). <\/td><\/tr><tr><th>Diarama <\/th><td>Moderately restores 1 ally's HP. <\/td><\/tr><tr><th>Magaru <\/th><td>Deals light Wind damage to all foes. <\/td><\/tr><\/table><\/td><\/tr><\/table><\/div><\/div><br><table class=\"collapsible collapsed\"><tr><th colspan=\"2\"><div><i><a href=\"\/wiki\/Persona_3\" title=\"Persona 3\"><font>Persona 3 Bosses<\/font><\/a><\/i><\/div> <\/th><\/tr><tr><td colspan=\"2\"><b><a href=\"\/wiki\/List_of_Persona_3_Bosses\" title=\"List of Persona 3 Bosses\">P3 Bosses<\/a><\/b> <\/td><\/tr><tr><td><b>Guardians<\/b> <\/td><td><a href=\"\/wiki\/Venus_Eagle\" title=\"Venus Eagle\">Venus Eagle<\/a> (3) - <a href=\"\/wiki\/Dancing_Hand\" title=\"Dancing Hand\">Dancing Hand<\/a> (3) - <a href=\"\/wiki\/Rampage_Drive\" title=\"Rampage Drive\">Rampage Drive<\/a> - <strong class=\"selflink\">Crying Table<\/strong> (3) - <a href=\"\/wiki\/Change_Relic\" title=\"Change Relic\">Change Relic<\/a> - <a href=\"\/wiki\/Golden_Beetle\" title=\"Golden Beetle\">Golden Beetle<\/a> (3) - <a href=\"\/wiki\/Intrepid_Knight\" title=\"Intrepid Knight\">Intrepid Knight<\/a> - <a href=\"\/wiki\/Furious_Gigas\" title=\"Furious Gigas\">Furious Gigas<\/a> (3) - <a href=\"\/wiki\/Fanatic_Tower\" title=\"Fanatic Tower\">Fanatic Tower<\/a> - <a href=\"\/wiki\/Magical_Magus\" title=\"Magical Magus\">Magical Magus<\/a> (3) - <a href=\"\/wiki\/Natural_Dancer\" title=\"Natural Dancer\">Natural Dancer<\/a> - <a href=\"\/wiki\/Arcane_Turret\" title=\"Arcane Turret\">Arcane Turret<\/a> (3) - <a href=\"\/wiki\/Sleeping_Table\" title=\"Sleeping Table\">Sleeping Table<\/a> - <a href=\"\/wiki\/Hell_Knight\" title=\"Hell Knight\">Hell Knight<\/a> (3) - <a href=\"\/wiki\/Mythical_Gigas\" title=\"Mythical Gigas\">Mythical Gigas<\/a> - <a href=\"\/wiki\/Judgement_Sword\" title=\"Judgement Sword\">Judgement Sword<\/a> (3) - <a href=\"\/wiki\/Stasis_Giant\" title=\"Stasis Giant\">Stasis Giant<\/a> (3) - <a href=\"\/wiki\/Phantom_King\" title=\"Phantom King\">Phantom King<\/a> - <a href=\"\/wiki\/Royal_Dancer\" title=\"Royal Dancer\">Royal Dancer<\/a> (3) - <a href=\"\/wiki\/Reckoning_Dice\" title=\"Reckoning Dice\">Reckoning Dice<\/a> - <a href=\"\/wiki\/Noble_Seeker\" title=\"Noble Seeker\">Noble Seeker<\/a> (3) - <a href=\"\/wiki\/Carnal_Snake\" title=\"Carnal Snake\">Carnal Snake<\/a> (3) - <a href=\"\/wiki\/World_Balance\" title=\"World Balance\">World Balance<\/a> - <a href=\"\/wiki\/Fierce_Cyclops\" title=\"Fierce Cyclops\">Fierce Cyclops<\/a> (3) - <a href=\"\/wiki\/Jotun_of_Grief\" title=\"Jotun of Grief\">Jotun of Grief<\/a> <\/td><\/tr><tr><td><b>Arcana<\/b> <\/td><td><a href=\"\/wiki\/Arcana_Priestess\" title=\"Arcana Priestess\">Arcana Priestess<\/a> - <a href=\"\/wiki\/Arcana_Emperor\" title=\"Arcana Emperor\">Arcana Emperor<\/a> and <a href=\"\/wiki\/Arcana_Empress\" title=\"Arcana Empress\">Arcana Empress<\/a> - <a href=\"\/wiki\/Arcana_Hierophant\" title=\"Arcana Hierophant\">Arcana Hierophant<\/a> - <a href=\"\/wiki\/Arcana_Lovers\" title=\"Arcana Lovers\">Arcana Lovers<\/a> - <a href=\"\/wiki\/Arcana_Chariot\" title=\"Arcana Chariot\">Arcana Chariot<\/a> and  <a href=\"\/wiki\/Arcana_Justice\" title=\"Arcana Justice\">Arcana Justice<\/a> -  <a href=\"\/wiki\/Arcana_Hermit\" title=\"Arcana Hermit\">Arcana Hermit<\/a> - <a href=\"\/wiki\/Arcana_Fortune\" title=\"Arcana Fortune\">Arcana Fortune<\/a> and <a href=\"\/wiki\/Arcana_Strength\" title=\"Arcana Strength\">Arcana Strength<\/a> - <a href=\"\/wiki\/Arcana_Hanged_Man\" title=\"Arcana Hanged Man\">Arcana Hanged Man<\/a> - <a href=\"\/wiki\/Nyx_Avatar\" title=\"Nyx Avatar\">Death<\/a> <\/td><\/tr><tr><td><b>Other<\/b> <\/td><td><a href=\"\/wiki\/Takaya_Sakaki\" title=\"Takaya Sakaki\">Takaya Sakaki<\/a> and <a href=\"\/wiki\/Hypnos\" title=\"Hypnos\">Hypnos<\/a> - <a href=\"\/wiki\/Jin_Shirato\" title=\"Jin Shirato\">Jin Shirato<\/a> and <a href=\"\/wiki\/Moros\" title=\"Moros\">Moros<\/a> - <a href=\"\/wiki\/Chidori_Yoshino\" title=\"Chidori Yoshino\">Chidori Yoshino<\/a> and <a href=\"\/wiki\/Medea\" title=\"Medea\">Medea<\/a> - <a href=\"\/wiki\/Reaper_(Persona)\" title=\"Reaper (Persona)\">Reaper<\/a> - <a href=\"\/wiki\/Elizabeth\" title=\"Elizabeth\">Elizabeth<\/a> <\/td><\/tr><tr><td><b>P3P Exclusive<\/b> <\/td><td><a href=\"\/wiki\/Theodore\" title=\"Theodore\">Theodore<\/a> - <a href=\"\/wiki\/Margaret\" title=\"Margaret\">Margaret<\/a> <\/td><\/tr><tr><td colspan=\"2\"><b><a href=\"\/wiki\/List_of_Persona_3_FES_Bosses\" title=\"List of Persona 3 FES Bosses\">P3 FES Bosses<\/a><\/b> <\/td><\/tr><tr><td><b>Guardians<\/b> <\/td><td><a href=\"\/wiki\/Immortal_Gigas\" title=\"Immortal Gigas\">Immortal Gigas<\/a> + <a href=\"\/wiki\/Visceral_Maya\" title=\"Visceral Maya\">Visceral Maya<\/a> (3) - <a href=\"\/wiki\/Brilliant_Cyclops\" title=\"Brilliant Cyclops\">Brilliant Cyclops<\/a> - <a href=\"\/wiki\/Raindrop_Castle\" title=\"Raindrop Castle\">Death Castle<\/a> + <a href=\"\/wiki\/El_Dorado_Beast\" title=\"El Dorado Beast\">El Dorado Beast<\/a> (2) - <a href=\"\/wiki\/Harem_Dancer\" title=\"Harem Dancer\">Harem Dancer<\/a> + <a href=\"\/wiki\/Merciless_Maya\" title=\"Merciless Maya\">Merciless Maya<\/a> (2) - <a href=\"\/wiki\/Judgement_Sword\" title=\"Judgement Sword\">Judgement Sword<\/a> + <a href=\"\/wiki\/Ice_Raven\" title=\"Ice Raven\">Ice Raven<\/a> + <a href=\"\/wiki\/Brave_Wheel\" title=\"Brave Wheel\">Brave Wheel<\/a> - <a href=\"\/wiki\/Primitive_Idol\" title=\"Primitive Idol\">Primitive Idol<\/a> + <a href=\"\/wiki\/Shouting_Tiara\" title=\"Shouting Tiara\">Shouting Tiara<\/a> + <a href=\"\/wiki\/Wrathful_Book\" title=\"Wrathful Book\">Wrathful Book<\/a> - <a href=\"\/wiki\/Wondrous_Magus\" title=\"Wondrous Magus\">Wondrous Magus<\/a> + <strong class=\"selflink\">Crying Table<\/strong> + <a href=\"\/wiki\/Cowardly_Maya\" title=\"Cowardly Maya\">Cowardly Maya<\/a> - <a href=\"\/wiki\/Neo_Minotaur\" title=\"Neo Minotaur\">Neo Minotaur<\/a> - <a href=\"\/wiki\/Spastic_Turret\" title=\"Spastic Turret\">Spastic Turret<\/a> + <a href=\"\/wiki\/Slaughter_Drive\" title=\"Slaughter Drive\">Slaughter Drive<\/a> (2) - <a href=\"\/wiki\/Conceited_Maya\" title=\"Conceited Maya\">Conceited Maya<\/a> - <a href=\"\/wiki\/Rebellious_Cyclops\" title=\"Rebellious Cyclops\">Rebellious Cyclops<\/a> + <a href=\"\/wiki\/Acheron_Seeker\" title=\"Acheron Seeker\">Acheron Seeker<\/a> (2) - <a href=\"\/wiki\/Rain_Wind_Musha\" title=\"Rain Wind Musha\">Tenjin Musha<\/a> + <a href=\"\/wiki\/Rain_End_Musha\" title=\"Rain End Musha\">Kaiden Musha<\/a> + <a href=\"\/wiki\/Onnen_Musha\" title=\"Onnen Musha\" class=\"mw-redirect\">Onnen Musha<\/a> <\/td><\/tr><tr><td><b>Other<\/b> <\/td><td><a href=\"\/wiki\/Metis\" title=\"Metis\">Metis<\/a> and <a href=\"\/wiki\/Psyche\" title=\"Psyche\">Psyche<\/a> - <a href=\"\/wiki\/Protagonist_(Persona_3)\" title=\"Protagonist (Persona 3)\">???<\/a> - <a href=\"\/wiki\/Akihiko_Sanada\" title=\"Akihiko Sanada\">Akihiko Sanada<\/a> and <a href=\"\/wiki\/Caesar\" title=\"Caesar\">Caesar<\/a> + <a href=\"\/wiki\/Ken_Amada\" title=\"Ken Amada\">Ken Amada<\/a> and <a href=\"\/wiki\/Kala-Nemi\" title=\"Kala-Nemi\">Kala-Nemi<\/a> - <a href=\"\/wiki\/Junpei_Iori\" title=\"Junpei Iori\">Junpei Iori<\/a> and <a href=\"\/wiki\/Trismegistus\" title=\"Trismegistus\">Trismegistus<\/a> + <a href=\"\/wiki\/Koromaru\" title=\"Koromaru\">Koromaru<\/a> and <a href=\"\/wiki\/Cerberus\" title=\"Cerberus\">Cerberus<\/a> - <a href=\"\/wiki\/Yukari_Takeba\" title=\"Yukari Takeba\">Yukari Takeba<\/a> and <a href=\"\/wiki\/Isis\" title=\"Isis\">Isis<\/a> + <a href=\"\/wiki\/Mitsuru_Kirijo\" title=\"Mitsuru Kirijo\">Mitsuru Kirijo<\/a> and <a href=\"\/wiki\/Artemisia\" title=\"Artemisia\">Artemisia<\/a> - <a href=\"\/wiki\/Erebus\" title=\"Erebus\">Erebus<\/a> <\/td><\/tr><tr><td colspan=\"2\"><div class=\"noprint plainlinks navbar\">This box: <a href=\"\/wiki\/Template:P3Bosses\" title=\"Template:P3Bosses\"><span title=\"View this template\">view<\/span><\/a> <span>•<\/span> <span title=\"Discuss this template\">talk<\/span> <span>•<\/span> <a  class=\"text\" href=\"https:\/\/megamitensei.fandom.com\/wiki\/Template:P3Bosses?action=edit\"><span title=\"Edit this template\">edit<\/span><\/a><\/div> <\/td><\/tr><\/table><\/section>","categories":[{"title":"Magician Arcana","url":"\/wiki\/Category:Magician_Arcana"},{"title":"Persona 3 Shadows","url":"\/wiki\/Category:Persona_3_Shadows"},{"title":"Persona 4 Shadows","url":"\/wiki\/Category:Persona_4_Shadows"},{"title":"Persona 4 Golden Shadows","url":"\/wiki\/Category:Persona_4_Golden_Shadows"},{"title":"Persona 3 Bosses","url":"\/wiki\/Category:Persona_3_Bosses"}],"languageLinks":[],"displayTitle":"Crying Table","heroImage":{"type":"image","url":"https:\/\/vignette.wikia.nocookie.net\/megamitensei\/images\/7\/7c\/17_crying_table.png\/revision\/latest?cb=20190709125622","fileUrl":"https:\/\/megamitensei.fandom.com\/wiki\/File:17_crying_table.png","fileName":"17_crying_table.png","title":"17 crying table.png","user":"ItsLaVolpe","mime":"image\/png","isVideo":false,"isOgg":false,"href":"https:\/\/vignette.wikia.nocookie.net\/megamitensei\/images\/7\/7c\/17_crying_table.png\/revision\/latest?cb=20190709125622","isLinkedByUser":false,"width":356,"height":458,"context":"infobox-hero-image","thumbnail4by5":"https:\/\/vignette.wikia.nocookie.net\/megamitensei\/images\/7\/7c\/17_crying_table.png\/revision\/latest\/top-crop\/width\/360\/height\/450?cb=20190709125622","thumbnail4by52x":"https:\/\/vignette.wikia.nocookie.net\/megamitensei\/images\/7\/7c\/17_crying_table.png\/revision\/latest\/top-crop\/width\/720\/height\/900?cb=20190709125622","thumbnail4by5Width":360,"thumbnail4by5Height":450,"thumbnail1by1":"https:\/\/vignette.wikia.nocookie.net\/megamitensei\/images\/7\/7c\/17_crying_table.png\/revision\/latest\/top-crop\/width\/360\/height\/360?cb=20190709125622","thumbnail1by1Size":360}}
```

yea seriously, it's barely better than just parsing through the user-facing wiki page.

Mozilla's CSS selectors crate gets the job done for parsing the stripped-down HTML from the response.
However, in my experience it wasn't the most ergonomic to use... stuff like `Selector::parse()` possibly returning a
`ParseError`, which does not implement `Error`, so I can't use the `?` operator.

now, a problem with the SMT wiki's format specifically is the lack of standardization on the Shadow pages.
Given two Shadows that appear in both FES Journey + Answer, for example, there might be a surprising number of inconsistencies
in how their information is laid out on the page despite both fitting the same basic template. Check the tests for
a whole bunch of examples of this.

finally, a bug that really wound me up was discovering that section headers aren't always spelled correctly. At this point
I just assume that any shadow page that isn't parsed properly will have some lovely surprise like that waiting for me

thanks for reading my blog post
