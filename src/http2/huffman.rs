#[derive(Debug)]
struct HuffmanEntry {
    code: u32,
    bit_length: u8,
}

const EOS: HuffmanEntry = HuffmanEntry {
    code: 0x3fffffff,
    bit_length: 30,
};

const HUFFMAN_TABLE: [HuffmanEntry; 256] = [
    HuffmanEntry {  
        code: 0x1ff8,   
        bit_length: 13, 
    },  
    HuffmanEntry {  
        code: 0x7fffd8, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0xfffffe2,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffe3,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffe4,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffe5,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffe6,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffe7,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffe8,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffea, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x3ffffffc,   
        bit_length: 30, 
    },  
    HuffmanEntry {  
        code: 0xfffffe9,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffea,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0x3ffffffd,   
        bit_length: 30, 
    },  
    HuffmanEntry {  
        code: 0xfffffeb,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffec,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffed,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffee,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffffef,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff0,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff1,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff2,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0x3ffffffe,   
        bit_length: 30, 
    },  
    HuffmanEntry {  
        code: 0xffffff3,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff4,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff5,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff6,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff7,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff8,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffff9,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffffa,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xffffffb,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0x14, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x3f8,    
        bit_length: 10, 
    },  
    HuffmanEntry {  
        code: 0x3f9,    
        bit_length: 10, 
    },  
    HuffmanEntry {  
        code: 0xffa,    
        bit_length: 12, 
    },  
    HuffmanEntry {  
        code: 0x1ff9,   
        bit_length: 13, 
    },  
    HuffmanEntry {  
        code: 0x15, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0xf8, 
        bit_length: 8,  
    },  
    HuffmanEntry {  
        code: 0x7fa,    
        bit_length: 11, 
    },  
    HuffmanEntry {  
        code: 0x3fa,    
        bit_length: 10, 
    },  
    HuffmanEntry {  
        code: 0x3fb,    
        bit_length: 10, 
    },  
    HuffmanEntry {  
        code: 0xf9, 
        bit_length: 8,  
    },  
    HuffmanEntry {  
        code: 0x7fb,    
        bit_length: 11, 
    },  
    HuffmanEntry {  
        code: 0xfa, 
        bit_length: 8,  
    },  
    HuffmanEntry {  
        code: 0x16, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x17, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x18, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x0,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x1,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x2,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x19, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x1a, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x1b, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x1c, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x1d, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x1e, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x1f, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x5c, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0xfb, 
        bit_length: 8,  
    },  
    HuffmanEntry {  
        code: 0x7ffc,   
        bit_length: 15, 
    },  
    HuffmanEntry {  
        code: 0x20, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0xffb,    
        bit_length: 12, 
    },  
    HuffmanEntry {  
        code: 0x3fc,    
        bit_length: 10, 
    },  
    HuffmanEntry {  
        code: 0x1ffa,   
        bit_length: 13, 
    },  
    HuffmanEntry {  
        code: 0x21, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x5d, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x5e, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x5f, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x60, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x61, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x62, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x63, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x64, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x65, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x66, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x67, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x68, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x69, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x6a, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x6b, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x6c, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x6d, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x6e, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x6f, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x70, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x71, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x72, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0xfc, 
        bit_length: 8,  
    },  
    HuffmanEntry {  
        code: 0x73, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0xfd, 
        bit_length: 8,  
    },  
    HuffmanEntry {  
        code: 0x1ffb,   
        bit_length: 13, 
    },  
    HuffmanEntry {  
        code: 0x7fff0,  
        bit_length: 19, 
    },  
    HuffmanEntry {  
        code: 0x1ffc,   
        bit_length: 13, 
    },  
    HuffmanEntry {  
        code: 0x3ffc,   
        bit_length: 14, 
    },  
    HuffmanEntry {  
        code: 0x22, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x7ffd,   
        bit_length: 15, 
    },  
    HuffmanEntry {  
        code: 0x3,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x23, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x4,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x24, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x5,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x25, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x26, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x27, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x6,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x74, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x75, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x28, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x29, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x2a, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x7,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x2b, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x76, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x2c, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x8,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x9,  
        bit_length: 5,  
    },  
    HuffmanEntry {  
        code: 0x2d, 
        bit_length: 6,  
    },  
    HuffmanEntry {  
        code: 0x77, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x78, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x79, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x7a, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x7b, 
        bit_length: 7,  
    },  
    HuffmanEntry {  
        code: 0x7ffe,   
        bit_length: 15, 
    },  
    HuffmanEntry {  
        code: 0x7fc,    
        bit_length: 11, 
    },  
    HuffmanEntry {  
        code: 0x3ffd,   
        bit_length: 14, 
    },  
    HuffmanEntry {  
        code: 0x1ffd,   
        bit_length: 13, 
    },  
    HuffmanEntry {  
        code: 0xffffffc,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0xfffe6,  
        bit_length: 20, 
    },  
    HuffmanEntry {  
        code: 0x3fffd2, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0xfffe7,  
        bit_length: 20, 
    },  
    HuffmanEntry {  
        code: 0xfffe8,  
        bit_length: 20, 
    },  
    HuffmanEntry {  
        code: 0x3fffd3, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x3fffd4, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x3fffd5, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7fffd9, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3fffd6, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7fffda, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffdb, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffdc, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffdd, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffde, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0xffffeb, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x7fffdf, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0xffffec, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0xffffed, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x3fffd7, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7fffe0, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0xffffee, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x7fffe1, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffe2, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffe3, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffe4, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x1fffdc, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x3fffd8, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7fffe5, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3fffd9, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7fffe6, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffe7, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0xffffef, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x3fffda, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x1fffdd, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0xfffe9,  
        bit_length: 20, 
    },  
    HuffmanEntry {  
        code: 0x3fffdb, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x3fffdc, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7fffe8, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffe9, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x1fffde, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x7fffea, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3fffdd, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x3fffde, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0xfffff0, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x1fffdf, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x3fffdf, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7fffeb, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffec, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x1fffe0, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x1fffe1, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x3fffe0, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x1fffe2, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x7fffed, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3fffe1, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7fffee, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x7fffef, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0xfffea,  
        bit_length: 20, 
    },  
    HuffmanEntry {  
        code: 0x3fffe2, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x3fffe3, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x3fffe4, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7ffff0, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3fffe5, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x3fffe6, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7ffff1, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe0,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe1,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0xfffeb,  
        bit_length: 20, 
    },  
    HuffmanEntry {  
        code: 0x7fff1,  
        bit_length: 19, 
    },  
    HuffmanEntry {  
        code: 0x3fffe7, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x7ffff2, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3fffe8, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x1ffffec,    
        bit_length: 25, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe2,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe3,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe4,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x7ffffde,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffdf,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe5,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0xfffff1, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x1ffffed,    
        bit_length: 25, 
    },  
    HuffmanEntry {  
        code: 0x7fff2,  
        bit_length: 19, 
    },  
    HuffmanEntry {  
        code: 0x1fffe3, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe6,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe0,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe1,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe7,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe2,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0xfffff2, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x1fffe4, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x1fffe5, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe8,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x3ffffe9,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0xffffffd,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe3,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe4,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe5,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0xfffec,  
        bit_length: 20, 
    },  
    HuffmanEntry {  
        code: 0xfffff3, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0xfffed,  
        bit_length: 20, 
    },  
    HuffmanEntry {  
        code: 0x1fffe6, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x3fffe9, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x1fffe7, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x1fffe8, 
        bit_length: 21, 
    },  
    HuffmanEntry {  
        code: 0x7ffff3, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3fffea, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x3fffeb, 
        bit_length: 22, 
    },  
    HuffmanEntry {  
        code: 0x1ffffee,    
        bit_length: 25, 
    },  
    HuffmanEntry {  
        code: 0x1ffffef,    
        bit_length: 25, 
    },  
    HuffmanEntry {  
        code: 0xfffff4, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0xfffff5, 
        bit_length: 24, 
    },  
    HuffmanEntry {  
        code: 0x3ffffea,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x7ffff4, 
        bit_length: 23, 
    },  
    HuffmanEntry {  
        code: 0x3ffffeb,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe6,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x3ffffec,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x3ffffed,    
        bit_length: 26, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe7,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe8,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffe9,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffea,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffeb,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0xffffffe,    
        bit_length: 28, 
    },  
    HuffmanEntry {  
        code: 0x7ffffec,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffed,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffee,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7ffffef,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x7fffff0,    
        bit_length: 27, 
    },  
    HuffmanEntry {  
        code: 0x3ffffee,    
        bit_length: 26, 
    },  
];


pub fn huffman_encode(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut current = 0u64;
    let mut bits_in_current = 0;

    for &byte in input {
        let entry = &HUFFMAN_TABLE[byte as usize];
        current <<= entry.bit_length;
        current |= entry.code as u64;
        bits_in_current += entry.bit_length;

        while bits_in_current >= 8 {
            bits_in_current -= 8;
            output.push((current >> bits_in_current) as u8);
        }
    }

    if bits_in_current > 0 {
        current <<= 8 - bits_in_current;
        output.push(current as u8);
    }

    output
}

pub fn huffman_decode(input: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut output = Vec::new();
    let mut current = 0u32;
    let mut bits_in_current = 0;

    for &byte in input {
        current = (current << 8) | (byte as u32);
        bits_in_current += 8;

        while bits_in_current >= 8 {
            let mut found = false;
            for (i, entry) in HUFFMAN_TABLE.iter().enumerate() {
                if entry.bit_length <= bits_in_current {
                    let mask = (1 << entry.bit_length) - 1;
                    if (current >> (bits_in_current - entry.bit_length)) & mask == entry.code {
                        output.push(i as u8);
                        bits_in_current -= entry.bit_length;
                        current &= (1 << bits_in_current) - 1;
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                return Err("Invalid Huffman code");
            }
        }
    }

    Ok(output)
}