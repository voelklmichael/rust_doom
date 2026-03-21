// sounds.h - sound effect and music identifiers

pub use crate::i_sound::*;

// Original: extern sfxinfo_t S_sfx[]
pub fn s_sfx() -> *mut SfxinfoT {
    todo!("S_sfx: extern variable")
}

// Original: extern musicinfo_t S_music[]
pub fn s_music() -> *mut MusicinfoT {
    todo!("S_music: extern variable")
}

/// Original: musicenum_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MusicenumT {
    MusNone,
    MusE1m1, MusE1m2, MusE1m3, MusE1m4, MusE1m5, MusE1m6, MusE1m7, MusE1m8, MusE1m9,
    MusE2m1, MusE2m2, MusE2m3, MusE2m4, MusE2m5, MusE2m6, MusE2m7, MusE2m8, MusE2m9,
    MusE3m1, MusE3m2, MusE3m3, MusE3m4, MusE3m5, MusE3m6, MusE3m7, MusE3m8, MusE3m9,
    MusInter, MusIntro, MusBunny, MusVictor, MusIntroa, MusRunnin, MusStalks, MusCountd,
    MusBetwee, MusDoom, MusTheDa, MusShawn, MusDdtblu, MusInCit, MusDead, MusStlks2,
    MusTheda2, MusDoom2, MusDdtbl2, MusRunni2, MusDead2, MusStlks3, MusRomero, MusShawn2,
    MusMessag, MusCount2, MusDdtbl3, MusAmpie, MusTheda3, MusAdrian, MusMessg2, MusRomer2,
    MusTense, MusShawn3, MusOpenin, MusEvil, MusUltima, MusReadM, MusDm2ttl, MusDm2int,
    Nummusic,
}

/// Original: sfxenum_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SfxenumT {
    SfxNone,
    SfxPistol, SfxShotgn, SfxSgcock, SfxDshtgn, SfxDbopn, SfxDbcls, SfxDbload,
    SfxPlasma, SfxBfg, SfxSawup, SfxSawidl, SfxSawful, SfxSawhit, SfxRlaunc, SfxRxplod,
    SfxFirsht, SfxFirxpl, SfxPstart, SfxPstop, SfxDoropn, SfxDorcls, SfxStnmov,
    SfxSwtchn, SfxSwtchx, SfxPlpain, SfxDmpain, SfxPopain, SfxVipain, SfxMnpain,
    SfxPepain, SfxSlop, SfxItemup, SfxWpnup, SfxOof, SfxTelept, SfxPosit1, SfxPosit2,
    SfxPosit3, SfxBgsit1, SfxBgsit2, SfxSgtsit, SfxCacsit, SfxBrssit, SfxCybsit,
    SfxSpisit, SfxBspsit, SfxKntsit, SfxVilsit, SfxMansit, SfxPesit, SfxSklatk,
    SfxSgtatk, SfxSkepch, SfxVilatk, SfxClaw, SfxSkeswg, SfxPldeth, SfxPdiehi,
    SfxPodth1, SfxPodth2, SfxPodth3, SfxBgdth1, SfxBgdth2, SfxSgtdth, SfxCacdth,
    SfxSkldth, SfxBrsdth, SfxCybdth, SfxSpidth, SfxBspdth, SfxVildth, SfxKntdth,
    SfxPedth, SfxSkedth, SfxPosact, SfxBgact, SfxDmact, SfxBspact, SfxBspwlk,
    SfxVilact, SfxNoway, SfxBarexp, SfxPunch, SfxHoof, SfxMetal, SfxChgun, SfxTink,
    SfxBdopn, SfxBdcls, SfxItmbk, SfxFlame, SfxFlamst, SfxGetpow, SfxBospit,
    SfxBoscub, SfxBossit, SfxBospn, SfxBosdth, SfxManatk, SfxMandth, SfxSssit,
    SfxSsdth, SfxKeenpn, SfxKeendt, SfxSkeact, SfxSkesit, SfxSkeatk, SfxRadio,
    Numsfx,
}
