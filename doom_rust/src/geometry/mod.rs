//! Geometry utilities - angles, coordinates, and spatial calculations.

mod tables;
mod tables_data;

pub use tables::{
    finecosine, finesine, finetangent, gammatable, slope_div, tantoangle, Angle, ANG1, ANG1_X,
    ANG45, ANG60, ANG90, ANG180, ANG270, ANG_MAX, ANGLETOFINESHIFT, DBITS, FINEANGLES, FINEMASK,
    SLOPEBITS, SLOPERANGE,
};
