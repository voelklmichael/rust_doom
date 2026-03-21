// d_englsh.h - printed strings for translation
// No dependencies (leaf module)

// D_Main.C
pub const D_DEVSTR: &str = "Development mode ON.\n";
pub const D_CDROM: &str = "CD-ROM Version: default.cfg from c:\\doomdata\n";

// M_Menu.C
pub const PRESSKEY: &str = "press a key.";
pub const PRESSYN: &str = "press y or n.";

// Original: #define QUITMSG
pub const QUITMSG: &str = "are you sure you want to\nquit this great game?";

// Original: #define LOADNET
pub const LOADNET: &str = concat!("you can't do load while in a net game!\n\n", "press a key.");

// Original: #define QLOADNET
pub const QLOADNET: &str = concat!("you can't quickload during a netgame!\n\n", "press a key.");

// Original: #define QSAVESPOT
pub const QSAVESPOT: &str = concat!(
    "you haven't picked a quicksave slot yet!\n\n",
    "press a key."
);

// Original: #define SAVEDEAD
pub const SAVEDEAD: &str = concat!("you can't save if you aren't playing!\n\n", "press a key.");

// Original: #define QSPROMPT
pub const QSPROMPT: &str = concat!(
    "quicksave over your game named\n\n'%s'?\n\n",
    "press y or n."
);

// Original: #define QLPROMPT
pub const QLPROMPT: &str = concat!(
    "do you want to quickload the game named\n\n'%s'?\n\n",
    "press y or n."
);

// Original: #define NEWGAME
pub const NEWGAME: &str = concat!(
    "you can't start a new game\n",
    "while in a network game.\n\n",
    "press a key."
);

// Original: #define NIGHTMARE
pub const NIGHTMARE: &str = concat!(
    "are you sure? this skill level\n",
    "isn't even remotely fair.\n\n",
    "press y or n."
);

// Original: #define SWSTRING
pub const SWSTRING: &str = concat!(
    "this is the shareware version of doom.\n\n",
    "you need to order the entire trilogy.\n\n",
    "press a key."
);

// Original: #define MSGOFF
pub const MSGOFF: &str = "Messages OFF";

// Original: #define MSGON
pub const MSGON: &str = "Messages ON";

// Original: #define NETEND
pub const NETEND: &str = concat!("you can't end a netgame!\n\n", "press a key.");

// Original: #define ENDGAME
pub const ENDGAME: &str = concat!(
    "are you sure you want to end the game?\n\n",
    "press y or n."
);

// Original: #define DOSY
pub const DOSY: &str = "(press y to quit to dos.)";

// Original: #define DETAILHI
pub const DETAILHI: &str = "High detail";

// Original: #define DETAILLO
pub const DETAILLO: &str = "Low detail";

// Original: #define GAMMALVL0
pub const GAMMALVL0: &str = "Gamma correction OFF";

// Original: #define GAMMALVL1
pub const GAMMALVL1: &str = "Gamma correction level 1";

// Original: #define GAMMALVL2
pub const GAMMALVL2: &str = "Gamma correction level 2";

// Original: #define GAMMALVL3
pub const GAMMALVL3: &str = "Gamma correction level 3";

// Original: #define GAMMALVL4
pub const GAMMALVL4: &str = "Gamma correction level 4";

// Original: #define EMPTYSTRING
pub const EMPTYSTRING: &str = "empty slot";

// P_inter.C
pub const GOTARMOR: &str = "Picked up the armor.";
pub const GOTMEGA: &str = "Picked up the MegaArmor!";

// Original: #define GOTHTHBONUS
pub const GOTHTHBONUS: &str = "Picked up a health bonus.";

// Original: #define GOTARMBONUS
pub const GOTARMBONUS: &str = "Picked up an armor bonus.";

// Original: #define GOTSTIM
pub const GOTSTIM: &str = "Picked up a stimpack.";

// Original: #define GOTMEDINEED
pub const GOTMEDINEED: &str = "Picked up a medikit that you REALLY need!";

// Original: #define GOTMEDIKIT
pub const GOTMEDIKIT: &str = "Picked up a medikit.";

// Original: #define GOTSUPER
pub const GOTSUPER: &str = "Supercharge!";

// Original: #define GOTBLUECARD
pub const GOTBLUECARD: &str = "Picked up a blue keycard.";

// Original: #define GOTYELWCARD
pub const GOTYELWCARD: &str = "Picked up a yellow keycard.";

// Original: #define GOTREDCARD
pub const GOTREDCARD: &str = "Picked up a red keycard.";

// Original: #define GOTBLUESKUL
pub const GOTBLUESKUL: &str = "Picked up a blue skull key.";

// Original: #define GOTYELWSKUL
pub const GOTYELWSKUL: &str = "Picked up a yellow skull key.";

// Original: #define GOTREDSKULL
pub const GOTREDSKULL: &str = "Picked up a red skull key.";

// Original: #define GOTINVUL
pub const GOTINVUL: &str = "Invulnerability!";

// Original: #define GOTBERSERK
pub const GOTBERSERK: &str = "Berserk!";

// Original: #define GOTINVIS
pub const GOTINVIS: &str = "Partial Invisibility";

// Original: #define GOTSUIT
pub const GOTSUIT: &str = "Radiation Shielding Suit";

// Original: #define GOTMAP
pub const GOTMAP: &str = "Computer Area Map";

// Original: #define GOTVISOR
pub const GOTVISOR: &str = "Light Amplification Visor";

// Original: #define GOTMSPHERE
pub const GOTMSPHERE: &str = "MegaSphere!";

// Original: #define GOTCLIP
pub const GOTCLIP: &str = "Picked up a clip.";

// Original: #define GOTCLIPBOX
pub const GOTCLIPBOX: &str = "Picked up a box of bullets.";

// Original: #define GOTROCKET
pub const GOTROCKET: &str = "Picked up a rocket.";

// Original: #define GOTROCKBOX
pub const GOTROCKBOX: &str = "Picked up a box of rockets.";

// Original: #define GOTCELL
pub const GOTCELL: &str = "Picked up an energy cell.";

// Original: #define GOTCELLBOX
pub const GOTCELLBOX: &str = "Picked up an energy cell pack.";

// Original: #define GOTSHELLS
pub const GOTSHELLS: &str = "Picked up 4 shotgun shells.";

// Original: #define GOTSHELLBOX
pub const GOTSHELLBOX: &str = "Picked up a box of shotgun shells.";

// Original: #define GOTBACKPACK
pub const GOTBACKPACK: &str = "Picked up a backpack full of ammo!";

// Original: #define GOTBFG9000
pub const GOTBFG9000: &str = "You got the BFG9000!  Oh, yes.";

// Original: #define GOTCHAINGUN
pub const GOTCHAINGUN: &str = "You got the chaingun!";

// Original: #define GOTCHAINSAW
pub const GOTCHAINSAW: &str = "A chainsaw!  Find some meat!";

// Original: #define GOTLAUNCHER
pub const GOTLAUNCHER: &str = "You got the rocket launcher!";

// Original: #define GOTPLASMA
pub const GOTPLASMA: &str = "You got the plasma gun!";

// Original: #define GOTSHOTGUN
pub const GOTSHOTGUN: &str = "You got the shotgun!";

// Original: #define GOTSHOTGUN2
pub const GOTSHOTGUN2: &str = "You got the super shotgun!";

// Original: #define PD_BLUEO
pub const PD_BLUEO: &str = "You need a blue key to activate this object";

// Original: #define PD_REDO
pub const PD_REDO: &str = "You need a red key to activate this object";

// Original: #define PD_YELLOWO
pub const PD_YELLOWO: &str = "You need a yellow key to activate this object";

// Original: #define PD_BLUEK
pub const PD_BLUEK: &str = "You need a blue key to open this door";

// Original: #define PD_REDK
pub const PD_REDK: &str = "You need a red key to open this door";

// Original: #define PD_YELLOWK
pub const PD_YELLOWK: &str = "You need a yellow key to open this door";

// Original: #define GGSAVED
pub const GGSAVED: &str = "game saved.";

// Original: #define HUSTR_MSGU
pub const HUSTR_MSGU: &str = "[Message unsent]";

// Original: #define HUSTR_E1M1
pub const HUSTR_E1M1: &str = "E1M1: Hangar";

// Original: #define HUSTR_E1M2
pub const HUSTR_E1M2: &str = "E1M2: Nuclear Plant";

// Original: #define HUSTR_E1M3
pub const HUSTR_E1M3: &str = "E1M3: Toxin Refinery";

// Original: #define HUSTR_E1M4
pub const HUSTR_E1M4: &str = "E1M4: Command Control";

// Original: #define HUSTR_E1M5
pub const HUSTR_E1M5: &str = "E1M5: Phobos Lab";

// Original: #define HUSTR_E1M6
pub const HUSTR_E1M6: &str = "E1M6: Central Processing";

// Original: #define HUSTR_E1M7
pub const HUSTR_E1M7: &str = "E1M7: Computer Station";

// Original: #define HUSTR_E1M8
pub const HUSTR_E1M8: &str = "E1M8: Phobos Anomaly";

// Original: #define HUSTR_E1M9
pub const HUSTR_E1M9: &str = "E1M9: Military Base";

// Original: #define HUSTR_E2M1
pub const HUSTR_E2M1: &str = "E2M1: Deimos Anomaly";

// Original: #define HUSTR_E2M2
pub const HUSTR_E2M2: &str = "E2M2: Containment Area";

// Original: #define HUSTR_E2M3
pub const HUSTR_E2M3: &str = "E2M3: Refinery";

// Original: #define HUSTR_E2M4
pub const HUSTR_E2M4: &str = "E2M4: Deimos Lab";

// Original: #define HUSTR_E2M5
pub const HUSTR_E2M5: &str = "E2M5: Command Center";

// Original: #define HUSTR_E2M6
pub const HUSTR_E2M6: &str = "E2M6: Halls of the Damned";

// Original: #define HUSTR_E2M7
pub const HUSTR_E2M7: &str = "E2M7: Spawning Vats";

// Original: #define HUSTR_E2M8
pub const HUSTR_E2M8: &str = "E2M8: Tower of Babel";

// Original: #define HUSTR_E2M9
pub const HUSTR_E2M9: &str = "E2M9: Fortress of Mystery";

// Original: #define HUSTR_E3M1
pub const HUSTR_E3M1: &str = "E3M1: Hell Keep";

// Original: #define HUSTR_E3M2
pub const HUSTR_E3M2: &str = "E3M2: Slough of Despair";

// Original: #define HUSTR_E3M3
pub const HUSTR_E3M3: &str = "E3M3: Pandemonium";

// Original: #define HUSTR_E3M4
pub const HUSTR_E3M4: &str = "E3M4: House of Pain";

// Original: #define HUSTR_E3M5
pub const HUSTR_E3M5: &str = "E3M5: Unholy Cathedral";

// Original: #define HUSTR_E3M6
pub const HUSTR_E3M6: &str = "E3M6: Mt. Erebus";

// Original: #define HUSTR_E3M7
pub const HUSTR_E3M7: &str = "E3M7: Limbo";

// Original: #define HUSTR_E3M8
pub const HUSTR_E3M8: &str = "E3M8: Dis";

// Original: #define HUSTR_E3M9
pub const HUSTR_E3M9: &str = "E3M9: Warrens";

// Original: #define HUSTR_E4M1
pub const HUSTR_E4M1: &str = "E4M1: Hell Beneath";

// Original: #define HUSTR_E4M2
pub const HUSTR_E4M2: &str = "E4M2: Perfect Hatred";

// Original: #define HUSTR_E4M3
pub const HUSTR_E4M3: &str = "E4M3: Sever The Wicked";

// Original: #define HUSTR_E4M4
pub const HUSTR_E4M4: &str = "E4M4: Unruly Evil";

// Original: #define HUSTR_E4M5
pub const HUSTR_E4M5: &str = "E4M5: They Will Repent";

// Original: #define HUSTR_E4M6
pub const HUSTR_E4M6: &str = "E4M6: Against Thee Wickedly";

// Original: #define HUSTR_E4M7
pub const HUSTR_E4M7: &str = "E4M7: And Hell Followed";

// Original: #define HUSTR_E4M8
pub const HUSTR_E4M8: &str = "E4M8: Unto The Cruel";

// Original: #define HUSTR_E4M9
pub const HUSTR_E4M9: &str = "E4M9: Fear";

// Original: #define HUSTR_1
pub const HUSTR_1: &str = "level 1: entryway";

// Original: #define HUSTR_2
pub const HUSTR_2: &str = "level 2: underhalls";

// Original: #define HUSTR_3
pub const HUSTR_3: &str = "level 3: the gantlet";

// Original: #define HUSTR_4
pub const HUSTR_4: &str = "level 4: the focus";

// Original: #define HUSTR_5
pub const HUSTR_5: &str = "level 5: the waste tunnels";

// Original: #define HUSTR_6
pub const HUSTR_6: &str = "level 6: the crusher";

// Original: #define HUSTR_7
pub const HUSTR_7: &str = "level 7: dead simple";

// Original: #define HUSTR_8
pub const HUSTR_8: &str = "level 8: tricks and traps";

// Original: #define HUSTR_9
pub const HUSTR_9: &str = "level 9: the pit";

// Original: #define HUSTR_10
pub const HUSTR_10: &str = "level 10: refueling base";

// Original: #define HUSTR_11
pub const HUSTR_11: &str = "level 11: 'o' of destruction!";

// Original: #define HUSTR_12
pub const HUSTR_12: &str = "level 12: the factory";

// Original: #define HUSTR_13
pub const HUSTR_13: &str = "level 13: downtown";

// Original: #define HUSTR_14
pub const HUSTR_14: &str = "level 14: the inmost dens";

// Original: #define HUSTR_15
pub const HUSTR_15: &str = "level 15: industrial zone";

// Original: #define HUSTR_16
pub const HUSTR_16: &str = "level 16: suburbs";

// Original: #define HUSTR_17
pub const HUSTR_17: &str = "level 17: tenements";

// Original: #define HUSTR_18
pub const HUSTR_18: &str = "level 18: the courtyard";

// Original: #define HUSTR_19
pub const HUSTR_19: &str = "level 19: the citadel";

// Original: #define HUSTR_20
pub const HUSTR_20: &str = "level 20: gotcha!";

// Original: #define HUSTR_21
pub const HUSTR_21: &str = "level 21: nirvana";

// Original: #define HUSTR_22
pub const HUSTR_22: &str = "level 22: the catacombs";

// Original: #define HUSTR_23
pub const HUSTR_23: &str = "level 23: barrels o' fun";

// Original: #define HUSTR_24
pub const HUSTR_24: &str = "level 24: the chasm";

// Original: #define HUSTR_25
pub const HUSTR_25: &str = "level 25: bloodfalls";

// Original: #define HUSTR_26
pub const HUSTR_26: &str = "level 26: the abandoned mines";

// Original: #define HUSTR_27
pub const HUSTR_27: &str = "level 27: monster condo";

// Original: #define HUSTR_28
pub const HUSTR_28: &str = "level 28: the spirit world";

// Original: #define HUSTR_29
pub const HUSTR_29: &str = "level 29: the living end";

// Original: #define HUSTR_30
pub const HUSTR_30: &str = "level 30: icon of sin";

// Original: #define HUSTR_31
pub const HUSTR_31: &str = "level 31: wolfenstein";

// Original: #define HUSTR_32
pub const HUSTR_32: &str = "level 32: grosse";

// Original: #define PHUSTR_1
pub const PHUSTR_1: &str = "level 1: congo";

// Original: #define PHUSTR_2
pub const PHUSTR_2: &str = "level 2: well of souls";

// Original: #define PHUSTR_3
pub const PHUSTR_3: &str = "level 3: aztec";

// Original: #define PHUSTR_4
pub const PHUSTR_4: &str = "level 4: caged";

// Original: #define PHUSTR_5
pub const PHUSTR_5: &str = "level 5: ghost town";

// Original: #define PHUSTR_6
pub const PHUSTR_6: &str = "level 6: baron's lair";

// Original: #define PHUSTR_7
pub const PHUSTR_7: &str = "level 7: caughtyard";

// Original: #define PHUSTR_8
pub const PHUSTR_8: &str = "level 8: realm";

// Original: #define PHUSTR_9
pub const PHUSTR_9: &str = "level 9: abattoire";

// Original: #define PHUSTR_10
pub const PHUSTR_10: &str = "level 10: onslaught";

// Original: #define PHUSTR_11
pub const PHUSTR_11: &str = "level 11: hunted";

// Original: #define PHUSTR_12
pub const PHUSTR_12: &str = "level 12: speed";

// Original: #define PHUSTR_13
pub const PHUSTR_13: &str = "level 13: the crypt";

// Original: #define PHUSTR_14
pub const PHUSTR_14: &str = "level 14: genesis";

// Original: #define PHUSTR_15
pub const PHUSTR_15: &str = "level 15: the twilight";

// Original: #define PHUSTR_16
pub const PHUSTR_16: &str = "level 16: the omen";

// Original: #define PHUSTR_17
pub const PHUSTR_17: &str = "level 17: compound";

// Original: #define PHUSTR_18
pub const PHUSTR_18: &str = "level 18: neurosphere";

// Original: #define PHUSTR_19
pub const PHUSTR_19: &str = "level 19: nme";

// Original: #define PHUSTR_20
pub const PHUSTR_20: &str = "level 20: the death domain";

// Original: #define PHUSTR_21
pub const PHUSTR_21: &str = "level 21: slayer";

// Original: #define PHUSTR_22
pub const PHUSTR_22: &str = "level 22: impossible mission";

// Original: #define PHUSTR_23
pub const PHUSTR_23: &str = "level 23: tombstone";

// Original: #define PHUSTR_24
pub const PHUSTR_24: &str = "level 24: the final frontier";

// Original: #define PHUSTR_25
pub const PHUSTR_25: &str = "level 25: the temple of darkness";

// Original: #define PHUSTR_26
pub const PHUSTR_26: &str = "level 26: bunker";

// Original: #define PHUSTR_27
pub const PHUSTR_27: &str = "level 27: anti-christ";

// Original: #define PHUSTR_28
pub const PHUSTR_28: &str = "level 28: the sewers";

// Original: #define PHUSTR_29
pub const PHUSTR_29: &str = "level 29: odyssey of noises";

// Original: #define PHUSTR_30
pub const PHUSTR_30: &str = "level 30: the gateway of hell";

// Original: #define PHUSTR_31
pub const PHUSTR_31: &str = "level 31: cyberden";

// Original: #define PHUSTR_32
pub const PHUSTR_32: &str = "level 32: go 2 it";

// Original: #define THUSTR_1
pub const THUSTR_1: &str = "level 1: system control";

// Original: #define THUSTR_2
pub const THUSTR_2: &str = "level 2: human bbq";

// Original: #define THUSTR_3
pub const THUSTR_3: &str = "level 3: power control";

// Original: #define THUSTR_4
pub const THUSTR_4: &str = "level 4: wormhole";

// Original: #define THUSTR_5
pub const THUSTR_5: &str = "level 5: hanger";

// Original: #define THUSTR_6
pub const THUSTR_6: &str = "level 6: open season";

// Original: #define THUSTR_7
pub const THUSTR_7: &str = "level 7: prison";

// Original: #define THUSTR_8
pub const THUSTR_8: &str = "level 8: metal";

// Original: #define THUSTR_9
pub const THUSTR_9: &str = "level 9: stronghold";

// Original: #define THUSTR_10
pub const THUSTR_10: &str = "level 10: redemption";

// Original: #define THUSTR_11
pub const THUSTR_11: &str = "level 11: storage facility";

// Original: #define THUSTR_12
pub const THUSTR_12: &str = "level 12: crater";

// Original: #define THUSTR_13
pub const THUSTR_13: &str = "level 13: nukage processing";

// Original: #define THUSTR_14
pub const THUSTR_14: &str = "level 14: steel works";

// Original: #define THUSTR_15
pub const THUSTR_15: &str = "level 15: dead zone";

// Original: #define THUSTR_16
pub const THUSTR_16: &str = "level 16: deepest reaches";

// Original: #define THUSTR_17
pub const THUSTR_17: &str = "level 17: processing area";

// Original: #define THUSTR_18
pub const THUSTR_18: &str = "level 18: mill";

// Original: #define THUSTR_19
pub const THUSTR_19: &str = "level 19: shipping/respawning";

// Original: #define THUSTR_20
pub const THUSTR_20: &str = "level 20: central processing";

// Original: #define THUSTR_21
pub const THUSTR_21: &str = "level 21: administration center";

// Original: #define THUSTR_22
pub const THUSTR_22: &str = "level 22: habitat";

// Original: #define THUSTR_23
pub const THUSTR_23: &str = "level 23: lunar mining project";

// Original: #define THUSTR_24
pub const THUSTR_24: &str = "level 24: quarry";

// Original: #define THUSTR_25
pub const THUSTR_25: &str = "level 25: baron's den";

// Original: #define THUSTR_26
pub const THUSTR_26: &str = "level 26: ballistyx";

// Original: #define THUSTR_27
pub const THUSTR_27: &str = "level 27: mount pain";

// Original: #define THUSTR_28
pub const THUSTR_28: &str = "level 28: heck";

// Original: #define THUSTR_29
pub const THUSTR_29: &str = "level 29: river styx";

// Original: #define THUSTR_30
pub const THUSTR_30: &str = "level 30: last call";

// Original: #define THUSTR_31
pub const THUSTR_31: &str = "level 31: pharaoh";

// Original: #define THUSTR_32
pub const THUSTR_32: &str = "level 32: caribbean";

// Original: #define HUSTR_CHATMACRO1
pub const HUSTR_CHATMACRO1: &str = "I'm ready to kick butt!";

// Original: #define HUSTR_CHATMACRO2
pub const HUSTR_CHATMACRO2: &str = "I'm OK.";

// Original: #define HUSTR_CHATMACRO3
pub const HUSTR_CHATMACRO3: &str = "I'm not looking too good!";

// Original: #define HUSTR_CHATMACRO4
pub const HUSTR_CHATMACRO4: &str = "Help!";

// Original: #define HUSTR_CHATMACRO5
pub const HUSTR_CHATMACRO5: &str = "You suck!";

// Original: #define HUSTR_CHATMACRO6
pub const HUSTR_CHATMACRO6: &str = "Next time, scumbag...";

// Original: #define HUSTR_CHATMACRO7
pub const HUSTR_CHATMACRO7: &str = "Come here!";

// Original: #define HUSTR_CHATMACRO8
pub const HUSTR_CHATMACRO8: &str = "I'll take care of it.";

// Original: #define HUSTR_CHATMACRO9
pub const HUSTR_CHATMACRO9: &str = "Yes";

// Original: #define HUSTR_CHATMACRO0
pub const HUSTR_CHATMACRO0: &str = "No";

// Original: #define HUSTR_TALKTOSELF1
pub const HUSTR_TALKTOSELF1: &str = "You mumble to yourself";

// Original: #define HUSTR_TALKTOSELF2
pub const HUSTR_TALKTOSELF2: &str = "Who's there?";

// Original: #define HUSTR_TALKTOSELF3
pub const HUSTR_TALKTOSELF3: &str = "You scare yourself";

// Original: #define HUSTR_TALKTOSELF4
pub const HUSTR_TALKTOSELF4: &str = "You start to rave";

// Original: #define HUSTR_TALKTOSELF5
pub const HUSTR_TALKTOSELF5: &str = "You've lost it...";

// Original: #define HUSTR_MESSAGESENT
pub const HUSTR_MESSAGESENT: &str = "[Message Sent]";

// Original: #define HUSTR_PLRGREEN
pub const HUSTR_PLRGREEN: &str = "Green: ";

// Original: #define HUSTR_PLRINDIGO
pub const HUSTR_PLRINDIGO: &str = "Indigo: ";

// Original: #define HUSTR_PLRBROWN
pub const HUSTR_PLRBROWN: &str = "Brown: ";

// Original: #define HUSTR_PLRRED
pub const HUSTR_PLRRED: &str = "Red: ";

// Original: #define HUSTR_KEYGREEN
pub const HUSTR_KEYGREEN: char = 'g';

// Original: #define HUSTR_KEYINDIGO
pub const HUSTR_KEYINDIGO: char = 'i';

// Original: #define HUSTR_KEYBROWN
pub const HUSTR_KEYBROWN: char = 'b';

// Original: #define HUSTR_KEYRED
pub const HUSTR_KEYRED: char = 'r';

// Original: #define AMSTR_FOLLOWON
pub const AMSTR_FOLLOWON: &str = "Follow Mode ON";

// Original: #define AMSTR_FOLLOWOFF
pub const AMSTR_FOLLOWOFF: &str = "Follow Mode OFF";

// Original: #define AMSTR_GRIDON
pub const AMSTR_GRIDON: &str = "Grid ON";

// Original: #define AMSTR_GRIDOFF
pub const AMSTR_GRIDOFF: &str = "Grid OFF";

// Original: #define AMSTR_MARKEDSPOT
pub const AMSTR_MARKEDSPOT: &str = "Marked Spot";

// Original: #define AMSTR_MARKSCLEARED
pub const AMSTR_MARKSCLEARED: &str = "All Marks Cleared";

// Original: #define STSTR_MUS
pub const STSTR_MUS: &str = "Music Change";

// Original: #define STSTR_NOMUS
pub const STSTR_NOMUS: &str = "IMPOSSIBLE SELECTION";

// Original: #define STSTR_DQDON
pub const STSTR_DQDON: &str = "Degreelessness Mode On";

// Original: #define STSTR_DQDOFF
pub const STSTR_DQDOFF: &str = "Degreelessness Mode Off";

// Original: #define STSTR_KFAADDED
pub const STSTR_KFAADDED: &str = "Very Happy Ammo Added";

// Original: #define STSTR_FAADDED
pub const STSTR_FAADDED: &str = "Ammo (no keys) Added";

// Original: #define STSTR_NCON
pub const STSTR_NCON: &str = "No Clipping Mode ON";

// Original: #define STSTR_NCOFF
pub const STSTR_NCOFF: &str = "No Clipping Mode OFF";

// Original: #define STSTR_BEHOLD
pub const STSTR_BEHOLD: &str = "inVuln, Str, Inviso, Rad, Allmap, or Lite-amp";

// Original: #define STSTR_BEHOLDX
pub const STSTR_BEHOLDX: &str = "Power-up Toggled";

// Original: #define STSTR_CHOPPERS
pub const STSTR_CHOPPERS: &str = "... doesn't suck - GM";

// Original: #define STSTR_CLEV
pub const STSTR_CLEV: &str = "Changing Level...";

// Original: #define E1TEXT
pub const E1TEXT: &str = concat!(
    "Once you beat the big badasses and\n",
    "clean out the moon base you're supposed\n",
    "to win, aren't you? Aren't you? Where's\n",
    "your fat reward and ticket home? What\n",
    "the hell is this? It's not supposed to\n",
    "end this way!\n",
    "\n",
    "It stinks like rotten meat, but looks\n",
    "like the lost Deimos base.  Looks like\n",
    "you're stuck on The Shores of Hell.\n",
    "The only way out is through.\n",
    "\n",
    "To continue the DOOM experience, play\n",
    "The Shores of Hell and its amazing\n",
    "sequel, Inferno!\n"
);

// Original: #define E2TEXT
pub const E2TEXT: &str = concat!(
    "You've done it! The hideous cyber-\n",
    "demon lord that ruled the lost Deimos\n",
    "moon base has been slain and you\n",
    "are triumphant! But ... where are\n",
    "you? You clamber to the edge of the\n",
    "moon and look down to see the awful\n",
    "truth.\n",
    "\n",
    "Deimos floats above Hell itself!\n",
    "You've never heard of anyone escaping\n",
    "from Hell, but you'll make the bastards\n",
    "sorry they ever heard of you! Quickly,\n",
    "you rappel down to  the surface of\n",
    "Hell.\n",
    "\n",
    "Now, it's on to the final chapter of\n",
    "DOOM! -- Inferno."
);

// Original: #define E3TEXT
pub const E3TEXT: &str = concat!(
    "The loathsome spiderdemon that\n",
    "masterminded the invasion of the moon\n",
    "bases and caused so much death has had\n",
    "its ass kicked for all time.\n",
    "\n",
    "A hidden doorway opens and you enter.\n",
    "You've proven too tough for Hell to\n",
    "contain, and now Hell at last plays\n",
    "fair -- for you emerge from the door\n",
    "to see the green fields of Earth!\n",
    "Home at last.\n",
    "\n",
    "You wonder what's been happening on\n",
    "Earth while you were battling evil\n",
    "unleashed. It's good that no Hell-\n",
    "spawn could have come through that\n",
    "door with you ..."
);

// Original: #define E4TEXT
pub const E4TEXT: &str = concat!(
    "the spider mastermind must have sent forth\n",
    "its legions of hellspawn before your\n",
    "final confrontation with that terrible\n",
    "beast from hell.  but you stepped forward\n",
    "and brought forth eternal damnation and\n",
    "suffering upon the horde as a true hero\n",
    "would in the face of something so evil.\n",
    "\n",
    "besides, someone was gonna pay for what\n",
    "happened to daisy, your pet rabbit.\n",
    "\n",
    "but now, you see spread before you more\n",
    "potential pain and gibbitude as a nation\n",
    "of demons run amok among our cities.\n",
    "\n",
    "next stop, hell on earth!"
);

// Original: #define C1TEXT
pub const C1TEXT: &str = concat!(
    "YOU HAVE ENTERED DEEPLY INTO THE INFESTED\n",
    "STARPORT. BUT SOMETHING IS WRONG. THE\n",
    "MONSTERS HAVE BROUGHT THEIR OWN REALITY\n",
    "WITH THEM, AND THE STARPORT'S TECHNOLOGY\n",
    "IS BEING SUBVERTED BY THEIR PRESENCE.\n",
    "\n",
    "AHEAD, YOU SEE AN OUTPOST OF HELL, A\n",
    "FORTIFIED ZONE. IF YOU CAN GET PAST IT,\n",
    "YOU CAN PENETRATE INTO THE HAUNTED HEART\n",
    "OF THE STARBASE AND FIND THE CONTROLLING\n",
    "SWITCH WHICH HOLDS EARTH'S POPULATION\n",
    "HOSTAGE."
);

// Original: #define C2TEXT
pub const C2TEXT: &str = concat!(
    "YOU HAVE WON! YOUR VICTORY HAS ENABLED\n",
    "HUMANKIND TO EVACUATE EARTH AND ESCAPE\n",
    "THE NIGHTMARE.  NOW YOU ARE THE ONLY\n",
    "HUMAN LEFT ON THE FACE OF THE PLANET.\n",
    "CANNIBAL MUTATIONS, CARNIVOROUS ALIENS,\n",
    "AND EVIL SPIRITS ARE YOUR ONLY NEIGHBORS.\n",
    "YOU SIT BACK AND WAIT FOR DEATH, CONTENT\n",
    "THAT YOU HAVE SAVED YOUR SPECIES.\n",
    "\n",
    "BUT THEN, EARTH CONTROL BEAMS DOWN A\n",
    "MESSAGE FROM SPACE: \"SENSORS HAVE LOCATED\n",
    "THE SOURCE OF THE ALIEN INVASION. IF YOU\n",
    "GO THERE, YOU MAY BE ABLE TO BLOCK THEIR\n",
    "ENTRY.  THE ALIEN BASE IS IN THE HEART OF\n",
    "YOUR OWN HOME CITY, NOT FAR FROM THE\n",
    "STARPORT.\" SLOWLY AND PAINFULLY YOU GET\n",
    "UP AND RETURN TO THE FRAY."
);

// Original: #define C3TEXT
pub const C3TEXT: &str = concat!(
    "YOU ARE AT THE CORRUPT HEART OF THE CITY,\n",
    "SURROUNDED BY THE CORPSES OF YOUR ENEMIES.\n",
    "YOU SEE NO WAY TO DESTROY THE CREATURES'\n",
    "ENTRYWAY ON THIS SIDE, SO YOU CLENCH YOUR\n",
    "TEETH AND PLUNGE THROUGH IT.\n",
    "\n",
    "THERE MUST BE A WAY TO CLOSE IT ON THE\n",
    "OTHER SIDE. WHAT DO YOU CARE IF YOU'VE\n",
    "GOT TO GO THROUGH HELL TO GET TO IT?"
);

// Original: #define C4TEXT
pub const C4TEXT: &str = concat!(
    "THE HORRENDOUS VISAGE OF THE BIGGEST\n",
    "DEMON YOU'VE EVER SEEN CRUMBLES BEFORE\n",
    "YOU, AFTER YOU PUMP YOUR ROCKETS INTO\n",
    "HIS EXPOSED BRAIN. THE MONSTER SHRIVELS\n",
    "UP AND DIES, ITS THRASHING LIMBS\n",
    "DEVASTATING UNTOLD MILES OF HELL'S\n",
    "SURFACE.\n",
    "\n",
    "YOU'VE DONE IT. THE INVASION IS OVER.\n",
    "EARTH IS SAVED. HELL IS A WRECK. YOU\n",
    "WONDER WHERE BAD FOLKS WILL GO WHEN THEY\n",
    "DIE, NOW. WIPING THE SWEAT FROM YOUR\n",
    "FOREHEAD YOU BEGIN THE LONG TREK BACK\n",
    "HOME. REBUILDING EARTH OUGHT TO BE A\n",
    "LOT MORE FUN THAN RUINING IT WAS.\n"
);

// Original: #define C5TEXT
pub const C5TEXT: &str = concat!(
    "CONGRATULATIONS, YOU'VE FOUND THE SECRET\n",
    "LEVEL! LOOKS LIKE IT'S BEEN BUILT BY\n",
    "HUMANS, RATHER THAN DEMONS. YOU WONDER\n",
    "WHO THE INMATES OF THIS CORNER OF HELL\n",
    "WILL BE."
);

// Original: #define C6TEXT
pub const C6TEXT: &str = concat!(
    "CONGRATULATIONS, YOU'VE FOUND THE\n",
    "SUPER SECRET LEVEL!  YOU'D BETTER\n",
    "BLAZE THROUGH THIS ONE!\n"
);

// Original: #define P1TEXT
pub const P1TEXT: &str = concat!(
    "You gloat over the steaming carcass of the\n",
    "Guardian.  With its death, you've wrested\n",
    "the Accelerator from the stinking claws\n",
    "of Hell.  You relax and glance around the\n",
    "room.  Damn!  There was supposed to be at\n",
    "least one working prototype, but you can't\n",
    "see it. The demons must have taken it.\n",
    "\n",
    "You must find the prototype, or all your\n",
    "struggles will have been wasted. Keep\n",
    "moving, keep fighting, keep killing.\n",
    "Oh yes, keep living, too."
);

// Original: #define P2TEXT
pub const P2TEXT: &str = concat!(
    "Even the deadly Arch-Vile labyrinth could\n",
    "not stop you, and you've gotten to the\n",
    "prototype Accelerator which is soon\n",
    "efficiently and permanently deactivated.\n",
    "\n",
    "You're good at that kind of thing."
);

// Original: #define P3TEXT
pub const P3TEXT: &str = concat!(
    "You've bashed and battered your way into\n",
    "the heart of the devil-hive.  Time for a\n",
    "Search-and-Destroy mission, aimed at the\n",
    "Gatekeeper, whose foul offspring is\n",
    "cascading to Earth.  Yeah, he's bad. But\n",
    "you know who's worse!\n",
    "\n",
    "Grinning evilly, you check your gear, and\n",
    "get ready to give the bastard a little Hell\n",
    "of your own making!"
);

// Original: #define P4TEXT
pub const P4TEXT: &str = concat!(
    "The Gatekeeper's evil face is splattered\n",
    "all over the place.  As its tattered corpse\n",
    "collapses, an inverted Gate forms and\n",
    "sucks down the shards of the last\n",
    "prototype Accelerator, not to mention the\n",
    "few remaining demons.  You're done. Hell\n",
    "has gone back to pounding bad dead folks \n",
    "instead of good live ones.  Remember to\n",
    "tell your grandkids to put a rocket\n",
    "launcher in your coffin. If you go to Hell\n",
    "when you die, you'll need it for some\n",
    "final cleaning-up ..."
);

// Original: #define P5TEXT
pub const P5TEXT: &str = concat!(
    "You've found the second-hardest level we\n",
    "got. Hope you have a saved game a level or\n",
    "two previous.  If not, be prepared to die\n",
    "aplenty. For master marines only."
);

// Original: #define P6TEXT
pub const P6TEXT: &str = concat!(
    "Betcha wondered just what WAS the hardest\n",
    "level we had ready for ya?  Now you know.\n",
    "No one gets out alive."
);

// Original: #define T1TEXT
pub const T1TEXT: &str = concat!(
    "You've fought your way out of the infested\n",
    "experimental labs.   It seems that UAC has\n",
    "once again gulped it down.  With their\n",
    "high turnover, it must be hard for poor\n",
    "old UAC to buy corporate health insurance\n",
    "nowadays..\n",
    "\n",
    "Ahead lies the military complex, now\n",
    "swarming with diseased horrors hot to get\n",
    "their teeth into you. With luck, the\n",
    "complex still has some warlike ordnance\n",
    "laying around."
);

// Original: #define T2TEXT
pub const T2TEXT: &str = concat!(
    "You hear the grinding of heavy machinery\n",
    "ahead.  You sure hope they're not stamping\n",
    "out new hellspawn, but you're ready to\n",
    "ream out a whole herd if you have to.\n",
    "They might be planning a blood feast, but\n",
    "you feel about as mean as two thousand\n",
    "maniacs packed into one mad killer.\n",
    "\n",
    "You don't plan to go down easy."
);

// Original: #define T3TEXT
pub const T3TEXT: &str = concat!(
    "The vista opening ahead looks real damn\n",
    "familiar. Smells familiar, too -- like\n",
    "fried excrement. You didn't like this\n",
    "place before, and you sure as hell ain't\n",
    "planning to like it now. The more you\n",
    "brood on it, the madder you get.\n",
    "Hefting your gun, an evil grin trickles\n",
    "onto your face. Time to take some names."
);

// Original: #define T4TEXT
pub const T4TEXT: &str = concat!(
    "Suddenly, all is silent, from one horizon\n",
    "to the other. The agonizing echo of Hell\n",
    "fades away, the nightmare sky turns to\n",
    "blue, the heaps of monster corpses start \n",
    "to evaporate along with the evil stench \n",
    "that filled the air. Jeeze, maybe you've\n",
    "done it. Have you really won?\n",
    "\n",
    "Something rumbles in the distance.\n",
    "A blue light begins to glow inside the\n",
    "ruined skull of the demon-spitter."
);

// Original: #define T5TEXT
pub const T5TEXT: &str = concat!(
    "What now? Looks totally different. Kind\n",
    "of like King Tut's condo. Well,\n",
    "whatever's here can't be any worse\n",
    "than usual. Can it?  Or maybe it's best\n",
    "to let sleeping gods lie.."
);

// Original: #define T6TEXT
pub const T6TEXT: &str = concat!(
    "Time for a vacation. You've burst the\n",
    "bowels of hell and by golly you're ready\n",
    "for a break. You mutter to yourself,\n",
    "Maybe someone else can kick Hell's ass\n",
    "next time around. Ahead lies a quiet town,\n",
    "with peaceful flowing water, quaint\n",
    "buildings, and presumably no Hellspawn.\n",
    "\n",
    "As you step off the transport, you hear\n",
    "the stomp of a cyberdemon's iron shoe."
);

// Original: #define CC_ZOMBIE
pub const CC_ZOMBIE: &str = "ZOMBIEMAN";

// Original: #define CC_SHOTGUN
pub const CC_SHOTGUN: &str = "SHOTGUN GUY";

// Original: #define CC_HEAVY
pub const CC_HEAVY: &str = "HEAVY WEAPON DUDE";

// Original: #define CC_IMP
pub const CC_IMP: &str = "IMP";

// Original: #define CC_DEMON
pub const CC_DEMON: &str = "DEMON";

// Original: #define CC_LOST
pub const CC_LOST: &str = "LOST SOUL";

// Original: #define CC_CACO
pub const CC_CACO: &str = "CACODEMON";

// Original: #define CC_HELL
pub const CC_HELL: &str = "HELL KNIGHT";

// Original: #define CC_BARON
pub const CC_BARON: &str = "BARON OF HELL";

// Original: #define CC_ARACH
pub const CC_ARACH: &str = "ARACHNOTRON";

// Original: #define CC_PAIN
pub const CC_PAIN: &str = "PAIN ELEMENTAL";

// Original: #define CC_REVEN
pub const CC_REVEN: &str = "REVENANT";

// Original: #define CC_MANCU
pub const CC_MANCU: &str = "MANCUBUS";

// Original: #define CC_ARCH
pub const CC_ARCH: &str = "ARCH-VILE";

// Original: #define CC_SPIDER
pub const CC_SPIDER: &str = "THE SPIDER MASTERMIND";

// Original: #define CC_CYBER
pub const CC_CYBER: &str = "THE CYBERDEMON";

// Original: #define CC_HERO
pub const CC_HERO: &str = "OUR HERO";
