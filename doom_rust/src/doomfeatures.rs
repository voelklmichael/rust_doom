//
// Copyright(C) 2005-2014 Simon Howard
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// DESCRIPTION:
//     List of features which can be enabled/disabled to slim down the
//     program.
//

// Original: doomfeatures.h

// Enables wad merging (the '-merge' command line parameter)
// C: #undef FEATURE_WAD_MERGE
pub const FEATURE_WAD_MERGE: bool = false;

// Enables dehacked support ('-deh')
// C: #undef FEATURE_DEHACKED
pub const FEATURE_DEHACKED: bool = false;

// Enables multiplayer support (network games)
// C: #undef FEATURE_MULTIPLAYER
pub const FEATURE_MULTIPLAYER: bool = false;

// Enables sound output
// C: //#undef FEATURE_SOUND (commented out = enabled)
pub const FEATURE_SOUND: bool = true;
