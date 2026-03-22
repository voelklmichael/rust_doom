//! Screen wipe effects (f_wipe.h, f_wipe.c)
//! Original: f_wipe.h, f_wipe.c

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WipeType {
    ColorXForm = 0,
    Melt = 1,
    NumWipes = 2,
}

pub struct F_WipeState;

impl F_WipeState {
    /// Original: int wipe_StartScreen(int x, int y, int width, int height)
    pub fn wipe_start_screen(&self, _x: i32, _y: i32, _width: i32, _height: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int wipe_EndScreen(int x, int y, int width, int height)
    pub fn wipe_end_screen(&self, _x: i32, _y: i32, _width: i32, _height: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int wipe_ScreenWipe(int wipeno, int x, int y, int width, int height, int ticks)
    pub fn wipe_screen_wipe(
        &self,
        _wipeno: i32,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
        _ticks: i32,
    ) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
