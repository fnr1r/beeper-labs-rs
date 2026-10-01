use super::prototypes::{CInt, XStr};

unsafe extern "C" {
    // http://www.manpagez.com/man/2/clonefileat/
    // https://github.com/apple/darwin-xnu/blob/0a798f6738bc1db01281fc08ae024145e84df927/bsd/sys/clonefile.h
    // TODO We need weak linkage here (OSX > 10.12, iOS > 10.0), otherwise compilation will fail on older versions
    pub(super) fn clonefile(src: XStr, dst: XStr, flags: CInt) -> CInt;
    pub(super) fn clonefileat(srcfd: CInt, src: XStr, dstfd: CInt, dst: XStr, flags: CInt) -> CInt;
    pub(super) fn fclonefileat(srcfd: CInt, dst_dirfd: CInt, dst: XStr, flags: CInt) -> CInt;
}
