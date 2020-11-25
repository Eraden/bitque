
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![],
  meta_content_scope: vec![],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(zoom|ycbcr2rgb|xyz2uint16|xyz2double|wiener2|whitepoint|watershed|warp|uintlut|uint8|uint16|truesize|translate|tonemap|tforminv|tformfwd|tformarray|subimage|stretchlim|strel|stdfilt|std2|roipoly|roifilt2|roifill|roicolor|rgbplot|rgb2ycbcr|rgb2ntsc|rgb2ind|rgb2hsv|rgb2gray|regionprops|reflect|rangefilt|radon|qtsetblk|qtgetblk|qtdecomp|psf2otf|poly2mask|pixval|phantom|para2fan|padarray|otf2psf|ordfilt2|ntsc2rgb|normxcorr2|nlfilter|nitfread|nitfinfo|montage|medfilt2|mean2|mat2gray|maketform|makeresampler|makelut|makecform|makeConstrainToRectFcn|label2rgb|lab2uint8|lab2uint16|lab2double|isrgb|isnitf|isind|isicc|isgray|isflat|isbw|iradon|iptwindowalign|iptsetpref|iptremovecallback|iptnum2ordinal|ipticondir|iptgetpref|iptgetapi|iptdemos|iptcheckstrs|iptchecknargin|iptcheckmap|iptcheckinput|iptcheckhandle|iptcheckconn|iptaddcallback|iptSetPointerBehavior|iptPointerManager|iptGetPointerBehavior|ippl|intlut|interfileread|interfileinfo|ind2rgb|ind2gray|imwrite|imview|imtransform|imtophat|imtool|imsubtract|imshow|imscrollpanel|imsave|imrotate|imresize|imregionalmin|imregionalmax|imrect|imreconstruct|imread|impyramid|imputfile|improfile|impositionrect|impoly|impoint|implay|impixelregionpanel|impixelregion|impixelinfoval|impixelinfo|impixel|imoverviewpanel|imoverview|imopen|imnoise|immultiply|immovie|immagbox|imline|imlincomb|imimposemin|imhmin|imhmax|imhist|imhandles|imgetfile|imgcf|imgca|imfreehand|imfinfo|imfilter|imfill|imextendedmin|imextendedmax|imerode|imellipse|imdivide|imdistline|imdisplayrange|imdilate|imcrop|imcontrast|imcontour|imcomplement|imclose|imclearborder|imbothat|imattributes|imapprox|imagemodel|imageinfo|imadjust|imadd|imabsdiff|im2uint8|im2uint16|im2single|im2java2d|im2java|im2int16|im2double|im2col|im2bw|ifftn|ifft2|ifanbeam|idct2|iccwrite|iccroot|iccread|iccfind|hsv2rgb|houghpeaks|houghlines|hough|histeq|hdrread|graythresh|grayslice|graycoprops|graycomatrix|gray2ind|getsequence|getrect|getrangefromclass|getpts|getnhood|getneighbors|getline|getimagemodel|getimage|getheight|fwind2|fwind1|ftrans2|fspecial|fsamp2|freqz2|freqspace|fliptform|findbounds|filter2|fftshift|fftn|fft2|fanbeam|fan2para|entropyfilt|entropy|edgetaper|edge|double|dither|dicomwrite|dicomuid|dicomread|dicomlookup|dicominfo|dicomdict|dicomanon|demosaic|decorrstretch|deconvwnr|deconvreg|deconvlucy|deconvblind|dctmtx|dct2|cpstruct2pairs|cpselect|cpcorr|cp2tform|corr2|convn|convmtx2|conv2|conndef|colorbar|colfilt|col2im|cmunique|cmpermute|checkerboard|bwunpack|bwulterode|bwtraceboundary|bwselect|bwperim|bwpack|bwmorph|bwlabeln|bwlabel|bwhitmiss|bweuler|bwdist|bwboundaries|bwareaopen|bwarea|brighten|blkproc|bestblk|axes2pix|applylut|applycform|analyze75read|analyze75info|adapthisteq)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044140,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }