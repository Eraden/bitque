use seed::prelude::*;
use seed::*;

use crate::Msg;

static LOGO: &str = include_str!("../../static/logo2.svg");

#[inline(always)]
pub fn render() -> Vec<Node<Msg>> {
    Node::from_html(Some(&Namespace::Svg), LOGO)
    // svg![
    //     attrs![
    //         "xmlns:xlink" => "http://www.w3.org/1999/xlink",
    //         At::Xmlns => "http://www.w3.org/2000/svg",
    //         At::ViewBox => "0 0 640 640",
    //         At::Width => "28",
    //         At::Height => "28"
    //     ],
    //     defs![
    //         path![attrs![
    //             At::D => "M500 300c0 110.38-89.62 200-200
    // 200s-200-89.62-200-200c0-110.39 89.62-200 200-200s200 89.61 200 200Z"
    //             At::Id => "a"
    //         ]],
    //         path![attrs![
    //             At::D => "M520 338.18c0 110.38-89.62 200-200
    // 200s-200-89.62-200-200 89.62-200 200-200 200 89.62 200 200Z",
    //             At::Id => "b"
    //         ]],
    //         path![attrs![
    //             At::D => "M543.03 374.84c0 110.39-89.62 200-200
    // 200s-200-89.61-200-200c0-110.38 89.62-200 200-200s200 89.62 200 200Z"\
    //             At::Id => "c"
    //         ]],
    //         mask![
    //             attrs![
    //                 At::Id => "e",
    //                 At::X => "78",
    //                 At::Y => "78",
    //                 At::Width => "600",
    //                 At::Height => "600",
    //                 At::MaskUnits => "userSpaceOnUse"
    //             ],
    //             path![attrs![
    //                 At::Fill => "#fff",
    //                 At::D => "M78 78h444v444H78z"
    //             ]]
    //             seed::r#use![attrs![
    //                 "xlink:href" => "#a",
    //                 At::Opacity => ".46"
    //             ]]
    //         ],
    //         mask![
    //             attrs![
    //                 At::Id => "f",
    //                 At::X => "98",
    //                 At::Y => "116.18",
    //                 At::Width => "444",
    //                 At::Height => "444",
    //                 At::MaskUnits => "userSpaceOnUse"
    //             ],
    //             path![attrs![
    //                 At::Fill => "#fff",
    //                 At::D => "M98 116.18h444v444H98z"
    //             ]],
    //             r#use![attrs![
    //                 "xlink:href" => "#b",
    //                 At::Opacity => ".46"
    //             ]],
    //         ],
    //         mask![
    //             attrs![
    //                 At::Id => "g",
    //                 At::X => "121.03",
    //                 At::Y => "152.84",
    //                 At::Width => "444",
    //                 At::Height => "444",
    //                 At::MaskUnits => "userSpaceOnUse"
    //             ],
    //             path![attrs![
    //                 At::Fill => "#fff",
    //                 At::D => "M121.03 152.84h444v444h-444z"
    //             ]],
    //             r#use![attrs![
    //                 "xlink:href" => "#c",
    //                 At::Opacity => ".46"
    //             ]],
    //         ],
    //         filter![
    //             attrs![At::Id => "d"],
    //             feFlood![],
    //             feComposite![attrs![
    //                 "in2" => "SourceAlpha",
    //                 "operator" => "in"
    //             ]],
    //             feGaussianBlur![attrs!["stdDeviation"=>"1"]],
    //             feOffset![attrs![At::Dx=>"14", At::Dy=>"10",
    // At::Result=>"afterOffset"]],
    // feFlood![attrs!["flood-color"=>"#0d0e44", "flood-opacity"=>".5"]],
    //             feComposite![attrs!["in2"=>"afterOffset", "operator"="in"]],
    //             feMorphology![attrs!["operator"=>"dilate", "radius"=>"1"]],
    //             feComposite![attrs!["in2"=>"SourceAlpha",
    // "operator"=>"out"]],         ]
    //     ],
    //     path![attrs![
    //         At::D=>"M500 300c0 110.38-89.62 200-200
    // 200s-200-89.62-200-200c0-110.39 89.62-200 200-200s200 89.61 200 200Z",
    //         At::Filter=>"url(#d)"
    //     ]],
    //     r#use![attrs!["xlink:href"=>"#a", "opacity"=>".46"
    // "fill"=>"#fefefe"]],     g![attrs!["mask"=>"url(#e)"]],
    //     r#use![attrs![[xlink:href="#a" opacity=".46" fill-opacity="0"
    // stroke="#06697d" stroke-width="44"/></g><path d="M520 338.18c0
    // 110.38-89.62 200-200 200s-200-89.62-200-200 89.62-200 200-200 200 89.62
    // 200 200Z" filter="url(#d)"/><use xlink:href="#b" opacity=".46"
    // fill="#fefefe"/><g mask="url(#f)"><use xlink:href="#b" opacity=".46"
    // fill-opacity="0" stroke="#06697d" stroke-width="44"/></g><path d="M543.03
    // 374.84c0 110.39-89.62 200-200 200s-200-89.61-200-200c0-110.38 89.62-200
    // 200-200s200 89.62 200 200Z" filter="url(#d)"/><use xlink:href="#c"
    // opacity=".46" fill="#fefefe"/><g mask="url(#g)"><use xlink:href="#c"
    // opacity=".46" fill-opacity="0" stroke="#06697d"
    // stroke-width="44"/></g></svg
}
