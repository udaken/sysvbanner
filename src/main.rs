// original: https://salsa.debian.org/debian/sysvbanner/-/blob/master/sysvbanner.c

/*****************************************************************
 *
 * There is no copyright or responsibility accepted for the use
 * of this software.
 *
 *****************************************************************/

use std::env;
use std::io::{stdout, BufWriter, Write};

const GLYPHS: &[&[u8]] = &[
    b"         ###  ### ###  # #   ##### ###   #  ##     ###  ",
    b"         ###  ### ###  # #  #  #  ## #  #  #  #    ###  ",
    b"         ###   #   # ########  #   ### #    ##      #   ",
    b"          #            # #   #####    #    ###     #    ",
    b"                     #######   #  #  # ####   # #       ",
    b"         ###           # #  #  #  # #  # ##    #        ",
    b"         ###           # #   ##### #   ### #### #       ",
    b"   ##    ##                                            #",
    b"  #        #   #   #    #                             # ",
    b" #          #   # #     #                            #  ",
    b" #          # ### ### #####   ###   #####           #   ",
    b" #          #   # #     #     ###           ###    #    ",
    b"  #        #   #   #    #      #            ###   #     ",
    b"   ##    ##                   #             ###  #      ",
    b"  ###     #    #####  ##### #      ####### ##### #######",
    b" #   #   ##   #     ##     ##    # #      #     ##    # ",
    b"# #   # # #         #      ##    # #      #          #  ",
    b"#  #  #   #    #####  ##### ####### ##### ######    #   ",
    b"#   # #   #   #            #     #       ##     #  #    ",
    b" #   #    #   #      #     #     # #     ##     #  #    ",
    b"  ###   ##### ####### #####      #  #####  #####   #    ",
    b" #####  #####    #     ###      #           #     ##### ",
    b"#     ##     #  # #    ###     #             #   #     #",
    b"#     ##     #   #            #     #####     #        #",
    b" #####  ######         ###   #                 #     ## ",
    b"#     #      #   #     ###    #     #####     #     #   ",
    b"#     ##     #  # #     #      #             #          ",
    b" #####  #####    #     #        #           #       #   ",
    b" #####    #   ######  ##### ###### ############## ##### ",
    b"#     #  # #  #     ##     ##     ##      #      #     #",
    b"# ### # #   # #     ##      #     ##      #      #      ",
    b"# # # ##     ####### #      #     ######  #####  #  ####",
    b"# #### ########     ##      #     ##      #      #     #",
    b"#     ##     ##     ##     ##     ##      #      #     #",
    b" ##### #     #######  ##### ###### ########       ##### ",
    b"#     #  #*#        ##    # #      #     ##     ########",
    b"#     #   #         ##   #  #      ##   ####    ##     #",
    b"#     #   #         ##  #   #      # # # ## #   ##     #",
    b"#######   #         ####    #      #  #  ##  #  ##     #",
    b"#     #   #   #     ##  #   #      #     ##   # ##     #",
    b"#     #   #   #     ##   #  #      #     ##    ###     #",
    b"#     #  ###   ##### #    # ########     ##     ########",
    b"######  ##### ######  ##### ########     ##     ##     #",
    b"#     ##     ##     ##     #   #   #     ##     ##  #  #",
    b"#     ##     ##     ##         #   #     ##     ##  #  #",
    b"###### #     #######  #####    #   #     ##     ##  #  #",
    b"#      #   # ##   #        #   #   #     # #   # #  #  #",
    b"#      #    # #    # #     #   #   #     #  # #  #  #  #",
    b"#       #### ##     # #####    #    #####    #    ## ## ",
    b"#     ##     ######## ##### #       #####    #          ",
    b" #   #  #   #      #  #      #          #   # #         ",
    b"  # #    # #      #   #       #         #  #   #        ",
    b"   #      #      #    #        #        #               ",
    b"  # #     #     #     #         #       #               ",
    b" #   #    #    #      #          #      #               ",
    b"#     #   #   ####### #####       # #####        #######",
    b"  ###                                                   ",
    b"  ###     ##   #####   ####  #####  ###### ######  #### ",
    b"   #     #  #  #    # #    # #    # #      #      #    #",
    b"    #   #    # #####  #      #    # #####  #####  #     ",
    b"        ###### #    # #      #    # #      #      #  ###",
    b"        #    # #    # #    # #    # #      #      #    #",
    b"        #    # #####   ####  #####  ###### #       #### ",
    b"                                                        ",
    b" #    #    #        # #    # #      #    # #    #  #### ",
    b" #    #    #        # #   #  #      ##  ## ##   # #    #",
    b" ######    #        # ####   #      # ## # # #  # #    #",
    b" #    #    #        # #  #   #      #    # #  # # #    #",
    b" #    #    #   #    # #   #  #      #    # #   ## #    #",
    b" #    #    #    ####  #    # ###### #    # #    #  #### ",
    b"                                                        ",
    b" #####   ####  #####   ####   ##### #    # #    # #    #",
    b" #    # #    # #    # #         #   #    # #    # #    #",
    b" #    # #    # #    #  ####     #   #    # #    # #    #",
    b" #####  #  # # #####       #    #   #    # #    # # ## #",
    b" #      #   #  #   #  #    #    #   #    #  #  #  ##  ##",
    b" #       ### # #    #  ####     #    ####    ##   #    #",
    b"                       ###     #     ###   ##    # # # #",
    b" #    #  #   # ###### #        #        # #  #  # # # # ",
    b"  #  #    # #      #  #        #        #     ## # # # #",
    b"   ##      #      #  ##                 ##        # # # ",
    b"   ##      #     #    #        #        #        # # # #",
    b"  #  #     #    #     #        #        #         # # # ",
    b" #    #    #   ######  ###     #     ###         # # # #",
];

const W: usize = 7;
fn main() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let args = env::args();
    for arg in args.skip(1) {
        let arg = arg.as_bytes();
        let len = arg.len().min(10);
        for a in 0..W {
            // line
            let mut line = [0u8; 80];
            for b in 0..len {
                let ind = if arg[b] >= b' ' { arg[b] - b' ' } else { 0 } as usize;
                let c = GLYPHS[(ind / 8 * W) + a];
                let src_start = b * 8;
                let dest_start = ind % 8 * W;
                line[src_start..(src_start + W)].clone_from_slice(&c[dest_start..(dest_start + W)]);
                line[src_start + W] = b' ';
            }

            let line = match line.iter().rposition(|&x| x != b' ') {
                Some(i) => &line[0..i + 1],
                None => &line,
            };
            out.write(&line).unwrap();
            out.write(b"\n").unwrap();
        }
        out.write(b"\n").unwrap();
    }
}
