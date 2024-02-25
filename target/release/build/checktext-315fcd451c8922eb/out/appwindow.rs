# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] # [allow (clippy :: overly_complex_bool_expr)] mod slint_generatedAppWindow {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#Lines {
         pub r#index : sp :: SharedString , pub r#number : sp :: SharedString }
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TextStyle {
         pub r#font_size : f32 , pub r#font_weight : i32 }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_68 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerColorSchemeSelector_68 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerCupertinoPalette_69 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerCupertinoPalette_69 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_70 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerStyleMetrics_70 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerTextEdit_root_45 {
         r#root_45 : sp :: r#Empty , r#_opacity_46 : sp :: r#Opacity , r#rectangle_47 : sp :: r#Rectangle , r#i_background_48 : sp :: r#BorderRectangle , r#i_scroll_view_49 : sp :: r#Empty , r#i_flickable_50 : sp :: r#Flickable , r#i_flickable_viewport_51 : sp :: r#Empty , r#i_text_input_52 : sp :: r#TextInput , r#i_vertical_bar_visibility_53 : sp :: r#Clip , r#i_vertical_bar_54 : sp :: r#Rectangle , r#i_border_55 : sp :: r#Rectangle , r#i_thumb_opacity_56 : sp :: r#Opacity , r#i_thumb_57 : sp :: r#BorderRectangle , r#i_touch_area_58 : sp :: r#TouchArea , r#i_horizontal_bar_visibility_59 : sp :: r#Clip , r#i_horizontal_bar_60 : sp :: r#Rectangle , r#i_border_61 : sp :: r#Rectangle , r#i_thumb_opacity_62 : sp :: r#Opacity , r#i_thumb_63 : sp :: r#BorderRectangle , r#i_touch_area_64 : sp :: r#TouchArea , r#root_45__opacity_46_dummy : sp :: Property < sp :: LogicalLength > , r#root_45_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_background_48_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_background_48_y : sp :: Property < sp :: LogicalLength > , r#root_45_i_flickable_50_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_flickable_50_width : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_60_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_60_maximum : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_60_pad : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_60_track_size : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_60_visible : sp :: Property < bool > , r#root_45_i_horizontal_bar_60_width : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_60_y : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_visibility_59_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_visibility_59_width : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_visibility_59_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_horizontal_bar_visibility_59_y : sp :: Property < sp :: LogicalLength > , r#root_45_i_text_input_52_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_text_input_52_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_45_i_text_input_52_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_text_input_52_y : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_57_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_57_width : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_57_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_57_y : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_63_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_63_width : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_63_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_63_y : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_opacity_56_dummy : sp :: Property < sp :: LogicalLength > , r#root_45_i_thumb_opacity_62_dummy : sp :: Property < sp :: LogicalLength > , r#root_45_i_touch_area_58_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_45_i_touch_area_58_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_touch_area_58_y : sp :: Property < sp :: LogicalLength > , r#root_45_i_touch_area_64_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_45_i_touch_area_64_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_touch_area_64_y : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_54_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_54_maximum : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_54_pad : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_54_track_size : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_54_visible : sp :: Property < bool > , r#root_45_i_vertical_bar_54_width : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_54_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_visibility_53_height : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_visibility_53_width : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_visibility_53_x : sp :: Property < sp :: LogicalLength > , r#root_45_i_vertical_bar_visibility_53_y : sp :: Property < sp :: LogicalLength > , r#root_45_rectangle_47_x : sp :: Property < sp :: LogicalLength > , r#root_45_rectangle_47_y : sp :: Property < sp :: LogicalLength > , r#root_45_state : sp :: Property < i32 > , r#root_45_width : sp :: Property < sp :: LogicalLength > , r#root_45_x : sp :: Property < sp :: LogicalLength > , r#root_45_y : sp :: Property < sp :: LogicalLength > , r#root_45_edited : sp :: Callback < (sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerTextEdit_root_45 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerTextEdit_root_45 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (16f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         20f64 }
                     else {
                         (12f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_pad }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         4f64 }
                     else {
                         (2f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_track_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) }
                     else {
                         (((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_visible }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_vertical_bar_54_maximum = ({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_vertical_bar_54_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_vertical_bar_54_page_size = ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_i_vertical_bar_54_page_size . clone () as f64) / (((r#tmp_i_vertical_bar_54_maximum . clone () as f64) + (r#tmp_i_vertical_bar_54_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! false {
                         ((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_vertical_bar_54_maximum = ({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_vertical_bar_54_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_vertical_bar_54_page_size = ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (r#tmp_i_vertical_bar_54_page_size . clone () as f64)) as f64) / (((r#tmp_i_vertical_bar_54_maximum . clone () as f64) + (r#tmp_i_vertical_bar_54_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! false {
                         ((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_horizontal_bar_60_maximum = ({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_horizontal_bar_60_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_horizontal_bar_60_page_size = ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_i_horizontal_bar_60_page_size . clone () as f64) / (((r#tmp_i_horizontal_bar_60_maximum . clone () as f64) + (r#tmp_i_horizontal_bar_60_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! true {
                         ((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_horizontal_bar_60_maximum = ({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_horizontal_bar_60_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_horizontal_bar_60_page_size = ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (r#tmp_i_horizontal_bar_60_page_size . clone () as f64)) as f64) / (((r#tmp_i_horizontal_bar_60_maximum . clone () as f64) + (r#tmp_i_horizontal_bar_60_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! true {
                         ((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_58_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_58_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_64_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_64_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_visible }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_pad }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         4f64 }
                     else {
                         (2f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_track_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) }
                     else {
                         (((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         20f64 }
                     else {
                         (12f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_rectangle_47_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_rectangle_47_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                         + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#_opacity_46 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#rectangle_47 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4282940159f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4284916195f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_background_48 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_45_state = ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_45_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (3009503585f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_45_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (4284572001f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (4281084972f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (83886080f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_background_48 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (654311423f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_background_48 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . get ()) == (sp :: r#TextWrap :: r#WordWrap)) {
                         ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         ((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_preferred_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64)) as f64) < (12f64 as f64)) {
                                 {
                                     ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                                    ) . apply_pin (_self) . get () . get () as f64) - (({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((- (args . 0 . clone ()) . r#x as f64) + (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64)) as f64) > (((({
                                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (12f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             ;
                             if (((((args . 0 . clone ()) . r#y as f64) + (({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64)) as f64) < (12f64 as f64)) {
                                 {
                                     ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                                    ) . apply_pin (_self) . get () . get () as f64) - (({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((- (args . 0 . clone ()) . r#y as f64) + (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#y as f64) + (({
                                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                                ) . apply_pin (_self) . get () . get () as f64) - (12f64 as f64)) as f64) - (20f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#y as f64)) as f64) - (12f64 as f64)) as f64) - (20f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (1291867601f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (1291877119f64 as u32)) as _ }
                    ) . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) . get () . color ()) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#TextWrap :: r#WordWrap) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_53 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_54 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_border_55 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_opacity_56 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.6f64) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_57 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_57 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_57 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((if false {
                         ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_height }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_width }
                        ) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_57 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (- (0f64 as sp :: Coord) . max (((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_58_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if false {
                                         ((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     else {
                                         (((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_58_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ((false) && ((((args . 0 . clone ()) . r#delta_x as f64) != (0f64 as f64)))) {
                                     (false , {
                                         ({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,) }
                                 else {
                                     (if ! ((! false) && ((((args . 0 . clone ()) . r#delta_y as f64) != (0f64 as f64)))) {
                                         {
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                         }
                                     else {
                                         ((false , {
                                             ({
                                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_maximum }
                                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                    ) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     ({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((sp :: r#EventResult :: r#Reject , false , (r#return_check_merge0 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression0 . clone ()) . 1 {
                                 (r#returned_expression0 . clone ()) . 0 }
                             else {
                                 ((r#returned_expression0 . clone ()) . 2) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_59 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_60 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_border_61 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_opacity_62 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.6f64) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_63 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_63 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_63 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((if true {
                         ({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_height }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_width }
                        ) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_63 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (- (0f64 as sp :: Coord) . max (((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_64_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if true {
                                         ((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     else {
                                         (((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_64_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- ({
                                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression1 = {
                                 let r#return_check_merge1 = if ((true) && ((((args . 0 . clone ()) . r#delta_x as f64) != (0f64 as f64)))) {
                                     (false , {
                                         ({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,) }
                                 else {
                                     (if ! ((! true) && ((((args . 0 . clone ()) . r#delta_y as f64) != (0f64 as f64)))) {
                                         {
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                         }
                                     else {
                                         ((false , {
                                             ({
                                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_maximum }
                                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                    ) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge1 . clone ()) . 0 {
                                     ({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((sp :: r#EventResult :: r#Reject , false , (r#return_check_merge1 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression1 . clone ()) . 1 {
                                 (r#returned_expression1 . clone ()) . 0 }
                             else {
                                 ((r#returned_expression1 . clone ()) . 2) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45__opacity_46_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_56_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_62_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_background_48 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_background_48 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#input_type) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_57 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_59 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_59 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_63 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             {
                 }
             ;
             {
                 {
                     }
                 ;
                 {
                     }
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_rectangle_47_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_rectangle_47_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (((({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as sp :: Coord , ((({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as sp :: Coord , 8f64 as sp :: Coord , 8f64 as sp :: Coord ,) , 4u32 => (((({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45__opacity_46_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45__opacity_46_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_flickable_50_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 2f64 as sp :: Coord , 2f64 as sp :: Coord ,) , 6u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 7u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 9u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 10u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord ,) , 11u32 => (if ! false {
                     ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                    ) . apply_pin (_self) . get () . get () }
                 else {
                     (0.8f64) as _ }
                 as sp :: Coord , if ! false {
                     0.8f64 }
                 else {
                     (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                    ) . apply_pin (_self) . get () . get ()) as _ }
                 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 12u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 13u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_54_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_58_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_58_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 14u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_57_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_56_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_56_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 15u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 16u32 => (if ! true {
                     ({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                    ) . apply_pin (_self) . get () . get () }
                 else {
                     (0.8f64) as _ }
                 as sp :: Coord , if ! true {
                     0.8f64 }
                 else {
                     (({
                         * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                    ) . apply_pin (_self) . get () . get ()) as _ }
                 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 17u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 18u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_60_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_64_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_touch_area_64_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 19u32 => (({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_63_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_62_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_62_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerButton_root_65 {
         r#root_65 : sp :: r#BorderRectangle , r#txt_66 : sp :: r#Text , r#touch_67 : sp :: r#TouchArea , r#root_65_touch_67_x : sp :: Property < sp :: LogicalLength > , r#root_65_touch_67_y : sp :: Property < sp :: LogicalLength > , r#root_65_txt_66_min_height : sp :: Property < sp :: LogicalLength > , r#root_65_txt_66_min_width : sp :: Property < sp :: LogicalLength > , r#root_65_txt_66_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_65_txt_66_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_65_txt_66_x : sp :: Property < sp :: LogicalLength > , r#root_65_txt_66_y : sp :: Property < sp :: LogicalLength > , r#root_65_width : sp :: Property < sp :: LogicalLength > , r#root_65_x : sp :: Property < sp :: LogicalLength > , r#root_65_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_65 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_65 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4285235842f64 as u32) }
                     else {
                         (if ({
                             * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             sp :: Color :: from_argb_encoded (4285292908f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4282668390f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
                     + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . get () . darker (0.25f64 as f32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as f64) + (if ({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         2f64 }
                     else {
                         (0f64) as _ }
                     as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as f64) - (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as f64) + (if ({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4293848814f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (18f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ((({
                             * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                        ) . apply_pin (_self) . get () . get () as f64) + (20f64 as f64)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((({
                             * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ((({
                             * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (((({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_1 : sp :: r#WindowItem , r#empty_3 : sp :: r#Empty , r#text_4 : sp :: r#Text , r#empty_5 : sp :: r#Empty , r#empty_6 : sp :: r#Empty , r#text_7 : sp :: r#Text , r#rectangle_9 : sp :: r#Empty , r#empty_11 : sp :: r#Empty , r#text_12 : sp :: r#Text , r#empty_14 : sp :: r#Empty , r#rectangle_15 : sp :: r#Empty , r#empty_17 : sp :: r#Empty , r#empty_18 : sp :: r#Empty , r#text_19 : sp :: r#Text , r#res_line_20 : sp :: r#Empty , r#i_flickable_21 : sp :: r#Flickable , r#i_flickable_viewport_22 : sp :: r#Empty , r#i_vertical_bar_visibility_26 : sp :: r#Clip , r#i_vertical_bar_27 : sp :: r#Rectangle , r#i_border_28 : sp :: r#Rectangle , r#i_thumb_opacity_29 : sp :: r#Opacity , r#i_thumb_30 : sp :: r#BorderRectangle , r#i_touch_area_31 : sp :: r#TouchArea , r#i_horizontal_bar_visibility_32 : sp :: r#Clip , r#i_horizontal_bar_33 : sp :: r#Rectangle , r#i_border_34 : sp :: r#Rectangle , r#i_thumb_opacity_35 : sp :: r#Opacity , r#i_thumb_36 : sp :: r#BorderRectangle , r#i_touch_area_37 : sp :: r#TouchArea , r#empty_38 : sp :: r#Empty , r#text_39 : sp :: r#Text , r#empty_42 : sp :: r#Empty , r#text_43 : sp :: r#Text , r#total_lines_44 : sp :: r#Text , r#b_config_8 : InnerTextEdit_root_45 , r#button_10 : InnerButton_root_65 , r#a_config_13 : InnerTextEdit_root_45 , r#button_16 : InnerButton_root_65 , r#button_40 : InnerButton_root_65 , r#res_check_41 : InnerTextEdit_root_45 , r#root_1_empty_11_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_11_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_11_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_14_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_14_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_14_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_14_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_17_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_17_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_17_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_18_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_18_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_18_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_2_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_2_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_38_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_38_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_38_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_42_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_42_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_42_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_5_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_5_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_5_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_6_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_6_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_6_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_i_flickable_21_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_flickable_21_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_33_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_33_maximum : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_33_pad : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_33_track_size : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_33_visible : sp :: Property < bool > , r#root_1_i_horizontal_bar_33_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_33_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_visibility_32_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_visibility_32_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_visibility_32_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_visibility_32_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_30_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_30_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_30_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_30_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_36_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_36_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_36_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_36_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_opacity_29_dummy : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_opacity_35_dummy : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_31_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_31_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_31_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_37_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_37_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_37_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_27_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_27_maximum : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_27_pad : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_27_track_size : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_27_visible : sp :: Property < bool > , r#root_1_i_vertical_bar_27_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_visibility_26_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_visibility_26_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_visibility_26_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_visibility_26_y : sp :: Property < sp :: LogicalLength > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_lines : sp :: Property < sp :: ModelRc < r#Lines > > , r#root_1_res : sp :: Property < sp :: SharedString > , r#root_1_res_backup : sp :: Property < sp :: ModelRc < sp :: SharedString > > , r#root_1_t_lines : sp :: Property < i32 > , r#root_1_text_39_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_text_39_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_refresh : sp :: Callback < () , () > , r#root_1_select : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_summit : sp :: Callback < (sp :: SharedString , sp :: SharedString ,) , () > , repeater0 : sp :: Repeater < InnerComponent_rectangle_23 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_AppWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerAppWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_lines }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             InnerTextEdit_root_45 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#b_config_8 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 9u32 - 1 , tree_index_of_first_child + 10u32 - 1) ;
             InnerButton_root_65 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_10 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 29u32 - 1 , tree_index_of_first_child + 30u32 - 1) ;
             InnerTextEdit_root_45 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#a_config_13 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 33u32 - 1 , tree_index_of_first_child + 34u32 - 1) ;
             InnerButton_root_65 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_16 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 56u32 - 1 , tree_index_of_first_child + 57u32 - 1) ;
             InnerButton_root_65 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_40 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 80u32 - 1 , tree_index_of_first_child + 81u32 - 1) ;
             InnerTextEdit_root_45 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#res_check_41 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 79u32 - 1 , tree_index_of_first_child + 83u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4280821800f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
                                 + {
                                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 250f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 250f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : {
                             let r#tmp_empty_5_padding = 12f64 ;
                             ;
                             ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                            ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                         as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 600f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 600f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 250f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 250f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
                                 + {
                                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                                ) . apply_pin (_self) . get () . get () as f64) + (20f64 as f64)) as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 1335f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 1335f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [5usize] as _ , r#spacing : 5f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 1f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 2f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [5usize] as _ , r#spacing : 5f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = ((({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
                             + {
                                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                            ) . apply_pin (_self) . get () . get () as f64) + (20f64 as f64)) as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 1335f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 1335f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 5f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 1f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 2f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 5f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 100f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 100f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 100f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 100f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#res_line_20 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : {
                             let r#tmp_empty_17_padding = 12f64 ;
                             ;
                             ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                            ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_17_padding . clone () as f64)) as f64) - (r#tmp_empty_17_padding . clone () as f64)) }
                         as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 100f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 100f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#res_line_20 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#res_line_20 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 10f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 5f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 1f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 2f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 10f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 5f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 5f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 10f64 as _ ;
                         the_struct . r#end = 10f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 1f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 2f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 5f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 10f64 as _ ;
                         the_struct . r#end = 10f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_39_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
                                 + {
                                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 250f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 250f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : {
                             let r#tmp_empty_17_padding = 12f64 ;
                             ;
                             ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                            ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_17_padding . clone () as f64)) as f64) - (r#tmp_empty_17_padding . clone () as f64)) }
                         as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_39_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 300f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 300f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 1235f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 1235f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_39_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 250f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 250f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 1230f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 1230f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 100f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 100f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : 1335f64 as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 1230f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 1230f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 100f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 100f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = ((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                                 + {
                                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                                ) . apply_pin (_self) . get () . get () as f64) + (20f64 as f64)) as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = ((({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                             + {
                                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                            ) . apply_pin (_self) . get () . get () as f64) + (20f64 as f64)) as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
                                 + {
                                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 250f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 250f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : {
                             let r#tmp_empty_5_padding = 12f64 ;
                             ;
                             ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                            ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                         as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 600f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 600f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 250f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 250f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * ({
                         let r#tmp_empty_18_padding = 12f64 ;
                         ;
                         ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                     as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         20f64 }
                     else {
                         (12f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_pad }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         4f64 }
                     else {
                         (2f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_track_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) }
                     else {
                         (((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_visible }
                    ) . apply_pin (_self) . get () {
                         (({
                             let r#tmp_empty_18_padding = 12f64 ;
                             ;
                             ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                         as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         ({
                             let r#tmp_empty_18_padding = 12f64 ;
                             ;
                             ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_vertical_bar_27_maximum = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_vertical_bar_27_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_vertical_bar_27_page_size = ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_i_vertical_bar_27_page_size . clone () as f64) / (((r#tmp_i_vertical_bar_27_maximum . clone () as f64) + (r#tmp_i_vertical_bar_27_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! false {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_vertical_bar_27_maximum = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_vertical_bar_27_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_vertical_bar_27_page_size = ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (r#tmp_i_vertical_bar_27_page_size . clone () as f64)) as f64) / (((r#tmp_i_vertical_bar_27_maximum . clone () as f64) + (r#tmp_i_vertical_bar_27_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! false {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_horizontal_bar_33_maximum = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_horizontal_bar_33_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_horizontal_bar_33_page_size = ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_i_horizontal_bar_33_page_size . clone () as f64) / (((r#tmp_i_horizontal_bar_33_maximum . clone () as f64) + (r#tmp_i_horizontal_bar_33_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! true {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_horizontal_bar_33_maximum = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_horizontal_bar_33_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_horizontal_bar_33_page_size = ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (r#tmp_i_horizontal_bar_33_page_size . clone () as f64)) as f64) / (((r#tmp_i_horizontal_bar_33_maximum . clone () as f64) + (r#tmp_i_horizontal_bar_33_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! true {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_31_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_31_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_37_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_37_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_visible }
                    ) . apply_pin (_self) . get () {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                        ) . apply_pin (_self) . get () [3usize]) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_pad }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         4f64 }
                     else {
                         (2f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_track_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) }
                     else {
                         (((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         20f64 }
                     else {
                         (12f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_lines }
            ) . apply_pin (_self) . set ({
                 (sp :: ModelRc :: new (sp :: VecModel :: < r#Lines > :: from (sp :: vec ! []))) as sp :: ModelRc < r#Lines > }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_res }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Please summit...")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_res_backup }
            ) . apply_pin (_self) . set ({
                 (sp :: ModelRc :: new (sp :: VecModel :: < sp :: SharedString > :: from (sp :: vec ! []))) as sp :: ModelRc < sp :: SharedString > }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_t_lines }
            ) . apply_pin (_self) . set ({
                 (0f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_39_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ((({
                             InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
                         + {
                             * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                        ) . apply_pin (_self) . get () . get () as f64) + (20f64 as f64)) as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_text_39_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Check Config")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (900f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("📖 Check Config Text")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (900f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Before")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_6_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_6_padding . clone () as f64)) as f64) - (r#tmp_empty_6_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (250f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (600f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
                 + {
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set (sp :: SharedString :: from ("") as _) ;
                             ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set (sp :: SharedString :: from ("") as _) ;
                             ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set (sp :: SharedString :: from ("Please summit...") as _) ;
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_lines }
                            ) . apply_pin (_self) . set (sp :: ModelRc :: new (sp :: VecModel :: < r#Lines > :: from (sp :: vec ! [])) as _) ;
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_res_backup }
                            ) . apply_pin (_self) . set (sp :: ModelRc :: new (sp :: VecModel :: < sp :: SharedString > :: from (sp :: vec ! [])) as _) ;
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
                             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set (sp :: SharedString :: from (sp :: format ! ("{}" , 0f64) . as_str ()) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Clear")) as sp :: SharedString }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (100f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (100f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         let r#tmp_empty_5_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                     as f64) - (((({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                     + {
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (900f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("After")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_11_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                        ) . apply_pin (_self) . get () [5usize] as f64) - (r#tmp_empty_11_padding . clone () as f64)) as f64) - (r#tmp_empty_11_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (250f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (600f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
                 + {
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_summit }
                            ) . apply_pin (_self) . call (& (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ , ({
                                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
                             + {
                                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Summit")) as sp :: SharedString }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (100f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (100f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (((({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
                     + {
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (900f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Lines")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (100f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_26 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_27 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_border_28 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_opacity_29 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.6f64) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_30 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_30 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_30 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((if false {
                         ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_height }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_width }
                        ) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (- (0f64 as sp :: Coord) . max (((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_31_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if false {
                                         ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     else {
                                         (((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_31_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression2 = {
                                 let r#return_check_merge2 = if ((false) && ((((args . 0 . clone ()) . r#delta_x as f64) != (0f64 as f64)))) {
                                     (false , {
                                         ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,) }
                                 else {
                                     (if ! ((! false) && ((((args . 0 . clone ()) . r#delta_y as f64) != (0f64 as f64)))) {
                                         {
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                         }
                                     else {
                                         ((false , {
                                             ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_maximum }
                                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                    ) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge2 . clone ()) . 0 {
                                     ({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((sp :: r#EventResult :: r#Reject , false , (r#return_check_merge2 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression2 . clone ()) . 1 {
                                 (r#returned_expression2 . clone ()) . 0 }
                             else {
                                 ((r#returned_expression2 . clone ()) . 2) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_32 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_33 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_border_34 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_opacity_35 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.6f64) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_36 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_36 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_36 }
                 + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((if true {
                         ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_height }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_width }
                        ) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_36 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (- (0f64 as sp :: Coord) . max (((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_37_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if true {
                                         ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     else {
                                         (((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_37_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression3 = {
                                 let r#return_check_merge3 = if ((true) && ((((args . 0 . clone ()) . r#delta_x as f64) != (0f64 as f64)))) {
                                     (false , {
                                         ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,) }
                                 else {
                                     (if ! ((! true) && ((((args . 0 . clone ()) . r#delta_y as f64) != (0f64 as f64)))) {
                                         {
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                         }
                                     else {
                                         ((false , {
                                             ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_maximum }
                                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                    ) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge3 . clone ()) . 0 {
                                     ({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((sp :: r#EventResult :: r#Reject , false , (r#return_check_merge3 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression3 . clone ()) . 1 {
                                 (r#returned_expression3 . clone ()) . 0 }
                             else {
                                 ((r#returned_expression3 . clone ()) . 2) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (900f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Result")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (300f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_refresh }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Refresh")) as sp :: SharedString }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (100f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (100f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (((({
                         InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
                     + {
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (250f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
                 + {
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_res }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1235f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
                 + {
                     * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (900f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_42_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                        ) . apply_pin (_self) . get () [5usize] as f64) - (r#tmp_empty_42_padding . clone () as f64)) as f64) - (r#tmp_empty_42_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Right) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Total number of lines : ")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1230f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4291962406f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (500f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_42_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                        ) . apply_pin (_self) . get () [5usize] as f64) - (r#tmp_empty_42_padding . clone () as f64)) as f64) - (r#tmp_empty_42_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: SharedString :: from (sp :: format ! ("{}" , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_t_lines }
                    ) . apply_pin (_self) . get ()) . as_str ())) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (100f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_32_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_32_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_32_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_32_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_29_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_35_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_26_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_26_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_26_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_26_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45__opacity_46_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_56_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_62_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45__opacity_46_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_56_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_62_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_26 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_26 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_30 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_32 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_32 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_36 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45__opacity_46_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_background_48_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_horizontal_bar_visibility_59_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_text_input_52_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_56_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_thumb_opacity_62_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_i_vertical_bar_visibility_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerTextEdit_root_45 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#b_config_8 }
             . apply_pin (x)) ,) ;
             InnerButton_root_65 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_10 }
             . apply_pin (x)) ,) ;
             InnerTextEdit_root_45 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#a_config_13 }
             . apply_pin (x)) ,) ;
             InnerButton_root_65 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_16 }
             . apply_pin (x)) ,) ;
             InnerButton_root_65 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_40 }
             . apply_pin (x)) ,) ;
             InnerTextEdit_root_45 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#res_check_41 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 {
                     {
                         }
                     ;
                     {
                         }
                     }
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated_listview (|| {
                         InnerComponent_rectangle_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                     , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                    ) . apply_pin (_self) . get () , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                    ) . apply_pin (_self)) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated_listview (|| {
                         InnerComponent_rectangle_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                     , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                    ) . apply_pin (_self) . get () , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                    ) . apply_pin (_self)) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated_listview (|| {
                         InnerComponent_rectangle_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                     , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                    ) . apply_pin (_self) . get () , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                    ) . apply_pin (_self)) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 4u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_3_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 5u32 => ({
                     let r#tmp_empty_5_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 6u32 => ({
                     let r#tmp_empty_5_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 7u32 => ({
                     let r#tmp_empty_5_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 8u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_6_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_6_padding . clone () as f64)) as f64) - (r#tmp_empty_6_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 9u32 => (250f64 as sp :: Coord , 600f64 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 29u32 => (((({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as sp :: Coord , 100f64 as sp :: Coord , ((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as f64) - (100f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 32u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_11_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as f64) - (r#tmp_empty_11_padding . clone () as f64)) as f64) - (r#tmp_empty_11_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 33u32 => (250f64 as sp :: Coord , 600f64 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 53u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 54u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 55u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , 1335f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 56u32 => (((({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as sp :: Coord , 100f64 as sp :: Coord , ((((({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_h }
                ) . apply_pin (_self) . get () [1usize] as f64) - (100f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 59u32 => ({
                     let r#tmp_empty_17_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_17_padding . clone () as f64)) as f64) - (r#tmp_empty_17_padding . clone () as f64)) }
                 as sp :: Coord , 100f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 60u32 => ({
                     let r#tmp_empty_17_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_17_padding . clone () as f64)) as f64) - (r#tmp_empty_17_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_17_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 61u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 100f64 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 62u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_18_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 63u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_flickable_21_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 2f64 as sp :: Coord , 2f64 as sp :: Coord ,) , 64u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_26_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_26_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_26_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_26_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 65u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_32_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_32_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_32_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_32_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 66u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 68u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , (({
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as f64) - (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , 0f64 as sp :: Coord ,) , 69u32 => (if ! false {
                     ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                    ) . apply_pin (_self) . get () . get () }
                 else {
                     (0.8f64) as _ }
                 as sp :: Coord , if ! false {
                     0.8f64 }
                 else {
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                    ) . apply_pin (_self) . get () . get ()) as _ }
                 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 70u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 71u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_27_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_31_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_31_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 72u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_30_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_29_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_29_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 73u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 74u32 => (if ! true {
                     ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                    ) . apply_pin (_self) . get () . get () }
                 else {
                     (0.8f64) as _ }
                 as sp :: Coord , if ! true {
                     0.8f64 }
                 else {
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                    ) . apply_pin (_self) . get () . get ()) as _ }
                 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 75u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 76u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_33_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_37_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_37_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 77u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_36_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_35_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_35_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 78u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 300f64 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 79u32 => (250f64 as sp :: Coord , 1235f64 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 80u32 => (((({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as sp :: Coord , 100f64 as sp :: Coord , 100f64 as sp :: Coord , ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 102u32 => ({
                     let r#tmp_empty_42_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                    ) . apply_pin (_self) . get () [5usize] as f64) - (r#tmp_empty_42_padding . clone () as f64)) as f64) - (r#tmp_empty_42_padding . clone () as f64)) }
                 as sp :: Coord , 1230f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 103u32 => ({
                     let r#tmp_empty_42_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_14_layout_cache_v }
                    ) . apply_pin (_self) . get () [5usize] as f64) - (r#tmp_empty_42_padding . clone () as f64)) as f64) - (r#tmp_empty_42_padding . clone () as f64)) }
                 as sp :: Coord , 100f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_42_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 10u32 ..= 28u32 => return {
                     * & Self :: FIELD_OFFSETS . r#b_config_8 }
                 . apply_pin (_self) . item_geometry (index - 10u32 + 1) , 30u32 ..= 31u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . item_geometry (index - 30u32 + 1) , 34u32 ..= 52u32 => return {
                     * & Self :: FIELD_OFFSETS . r#a_config_13 }
                 . apply_pin (_self) . item_geometry (index - 34u32 + 1) , 57u32 ..= 58u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_16 }
                 . apply_pin (_self) . item_geometry (index - 57u32 + 1) , 81u32 ..= 82u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . item_geometry (index - 81u32 + 1) , 83u32 ..= 101u32 => return {
                     * & Self :: FIELD_OFFSETS . r#res_check_41 }
                 . apply_pin (_self) . item_geometry (index - 83u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 => sp :: r#AccessibleRole :: r#Text , 8u32 => sp :: r#AccessibleRole :: r#Text , 32u32 => sp :: r#AccessibleRole :: r#Text , 61u32 => sp :: r#AccessibleRole :: r#Text , 78u32 => sp :: r#AccessibleRole :: r#Text , 102u32 => sp :: r#AccessibleRole :: r#Text , 103u32 => sp :: r#AccessibleRole :: r#Text , 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#b_config_8 }
                 . apply_pin (_self) . accessible_role (0) , 10u32 ..= 28u32 => {
                     * & Self :: FIELD_OFFSETS . r#b_config_8 }
                 . apply_pin (_self) . accessible_role (index - 10u32 + 1) , 29u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . accessible_role (0) , 30u32 ..= 31u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . accessible_role (index - 30u32 + 1) , 33u32 => {
                     * & Self :: FIELD_OFFSETS . r#a_config_13 }
                 . apply_pin (_self) . accessible_role (0) , 34u32 ..= 52u32 => {
                     * & Self :: FIELD_OFFSETS . r#a_config_13 }
                 . apply_pin (_self) . accessible_role (index - 34u32 + 1) , 56u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_16 }
                 . apply_pin (_self) . accessible_role (0) , 57u32 ..= 58u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_16 }
                 . apply_pin (_self) . accessible_role (index - 57u32 + 1) , 80u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessible_role (0) , 81u32 ..= 82u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessible_role (index - 81u32 + 1) , 79u32 => {
                     * & Self :: FIELD_OFFSETS . r#res_check_41 }
                 . apply_pin (_self) . accessible_role (0) , 83u32 ..= 101u32 => {
                     * & Self :: FIELD_OFFSETS . r#res_check_41 }
                 . apply_pin (_self) . accessible_role (index - 83u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (4u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("📖 Check Config Text") , (8u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Before") , (32u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("After") , (61u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Lines") , (78u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Result") , (102u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Total number of lines : ") , (103u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#b_config_8 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (10u32 ..= 28u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#b_config_8 }
                 . apply_pin (_self) . accessible_string_property (index - 10u32 + 1 , what) , (29u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (30u32 ..= 31u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . accessible_string_property (index - 30u32 + 1 , what) , (33u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#a_config_13 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (34u32 ..= 52u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#a_config_13 }
                 . apply_pin (_self) . accessible_string_property (index - 34u32 + 1 , what) , (56u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_16 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (57u32 ..= 58u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_16 }
                 . apply_pin (_self) . accessible_string_property (index - 57u32 + 1 , what) , (80u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (81u32 ..= 82u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_40 }
                 . apply_pin (_self) . accessible_string_property (index - 81u32 + 1 , what) , (79u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#res_check_41 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (83u32 ..= 101u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#res_check_41 }
                 . apply_pin (_self) . accessible_string_property (index - 83u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_23 {
         r#rectangle_23 : sp :: r#Empty , r#button_24 : InnerButton_root_65 , r#model_data : sp :: Property < r#Lines > , r#model_index : sp :: Property < i32 > , r#rectangle_23_height : sp :: Property < sp :: LogicalLength > , r#rectangle_23_width : sp :: Property < sp :: LogicalLength > , r#rectangle_23_x : sp :: Property < sp :: LogicalLength > , r#rectangle_23_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_23 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_23 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerButton_root_65 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_24 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 1u32 - 1 , tree_index_of_first_child + 2u32 - 1) ;
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_1_select) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& ((({
                                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#model_data }
                            ) . apply_pin (_self) . get ()) . r#index as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#number) as _ }
                ) ;
                 }
             ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ((({
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as f64) / (2f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ((((({
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as f64) - ((({
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as f64) / (2f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((30f64 as f64) - (((({
                         InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
                     + {
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_touch_67_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerButton_root_65 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_24 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = ((({
                         InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
                     + {
                         * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (20f64 as f64)) as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (30f64 as sp :: Coord , {
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as sp :: Coord , (({
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as f64) / (2f64 as f64)) as sp :: Coord , (((({
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as f64) - ((({
                     let r#tmp_empty_18_padding = 12f64 ;
                     ;
                     ((((100f64 as f64) - (r#tmp_empty_18_padding . clone () as f64)) as f64) - (r#tmp_empty_18_padding . clone () as f64)) }
                 as f64) / (2f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((30f64 as f64) - (((({
                     InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
                 + {
                     * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65_txt_66_preferred_height }
                ) . apply_pin (_self) . get () . get () as f64) * (1.33f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 2u32 ..= 3u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_24 }
                 . apply_pin (_self) . item_geometry (index - 2u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_24 }
                 . apply_pin (_self) . accessible_role (0) , 2u32 ..= 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_24 }
                 . apply_pin (_self) . accessible_role (index - 2u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_24 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (2u32 ..= 3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_24 }
                 . apply_pin (_self) . accessible_string_property (index - 2u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_rectangle_23 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             4usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 4u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 4u32 , parent_index : 1u32 , item_array_index : 3u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_23 , sp :: ItemVTable , sp :: AllowPin > ;
             4usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#button_24 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     impl sp :: PinnedDrop for InnerComponent_rectangle_23 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_rectangle_23 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_23) ;
             new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_23 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_23 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 67u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_23 {
         type Data = r#Lines ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_23 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn listview_layout (self : core :: pin :: Pin < & Self > , offset_y : & mut sp :: LogicalLength , viewport_width : core :: pin :: Pin < & sp :: Property < sp :: LogicalLength >> ,) {
             let _self = self ;
             let vp_w = viewport_width . get () ;
             ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_y }
            ) . apply_pin (_self) . set (* offset_y) ;
             * offset_y += ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_height }
            ) . apply_pin (_self) . get () ;
             let w = ({
                 * & InnerComponent_rectangle_23 :: FIELD_OFFSETS . r#rectangle_23_width }
            ) . apply_pin (_self) . get () ;
             if vp_w < w {
                 viewport_width . set (w) ;
                 }
             }
         }
     impl InnerAppWindow {
         pub fn new () -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & self_rc) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             104usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 53u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 8u32 , parent_index : 2u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 29u32 , parent_index : 2u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 32u32 , parent_index : 2u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 10u32 , parent_index : 5u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 10u32 , parent_index : 5u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 13u32 , parent_index : 9u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 9u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 14u32 , parent_index : 9u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 10u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 17u32 , parent_index : 12u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 19u32 , parent_index : 12u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 24u32 , parent_index : 12u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 18u32 , parent_index : 14u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 19u32 , parent_index : 17u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 20u32 , parent_index : 15u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 23u32 , parent_index : 19u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 23u32 , parent_index : 19u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 24u32 , parent_index : 19u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 24u32 , parent_index : 21u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 25u32 , parent_index : 16u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 24u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 28u32 , parent_index : 24u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 29u32 , parent_index : 24u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 29u32 , parent_index : 26u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 30u32 , parent_index : 6u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 32u32 , parent_index : 29u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 32u32 , parent_index : 29u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 34u32 , parent_index : 7u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 34u32 , parent_index : 7u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 37u32 , parent_index : 33u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 38u32 , parent_index : 33u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 38u32 , parent_index : 33u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 38u32 , parent_index : 34u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 41u32 , parent_index : 36u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 43u32 , parent_index : 36u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 48u32 , parent_index : 36u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 42u32 , parent_index : 38u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 41u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 44u32 , parent_index : 39u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 47u32 , parent_index : 43u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 47u32 , parent_index : 43u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 48u32 , parent_index : 43u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 48u32 , parent_index : 45u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 49u32 , parent_index : 40u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 52u32 , parent_index : 48u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 52u32 , parent_index : 48u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 53u32 , parent_index : 48u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 53u32 , parent_index : 50u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 56u32 , parent_index : 3u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 59u32 , parent_index : 3u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 102u32 , parent_index : 3u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 57u32 , parent_index : 53u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 59u32 , parent_index : 56u32 , item_array_index : 57u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 59u32 , parent_index : 56u32 , item_array_index : 58u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 61u32 , parent_index : 54u32 , item_array_index : 59u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 78u32 , parent_index : 54u32 , item_array_index : 60u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 63u32 , parent_index : 59u32 , item_array_index : 61u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 63u32 , parent_index : 59u32 , item_array_index : 62u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 66u32 , parent_index : 62u32 , item_array_index : 63u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 68u32 , parent_index : 62u32 , item_array_index : 64u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 73u32 , parent_index : 62u32 , item_array_index : 65u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 67u32 , parent_index : 63u32 , item_array_index : 66u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 66u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 69u32 , parent_index : 64u32 , item_array_index : 67u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 72u32 , parent_index : 68u32 , item_array_index : 68u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 72u32 , parent_index : 68u32 , item_array_index : 69u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 73u32 , parent_index : 68u32 , item_array_index : 70u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 73u32 , parent_index : 70u32 , item_array_index : 71u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 74u32 , parent_index : 65u32 , item_array_index : 72u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 77u32 , parent_index : 73u32 , item_array_index : 73u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 77u32 , parent_index : 73u32 , item_array_index : 74u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 78u32 , parent_index : 73u32 , item_array_index : 75u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 78u32 , parent_index : 75u32 , item_array_index : 76u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 1u32 , children_index : 80u32 , parent_index : 60u32 , item_array_index : 77u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 83u32 , parent_index : 60u32 , item_array_index : 78u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 81u32 , parent_index : 78u32 , item_array_index : 79u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 83u32 , parent_index : 80u32 , item_array_index : 80u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 83u32 , parent_index : 80u32 , item_array_index : 81u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 86u32 , parent_index : 79u32 , item_array_index : 82u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 87u32 , parent_index : 79u32 , item_array_index : 83u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 87u32 , parent_index : 79u32 , item_array_index : 84u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 87u32 , parent_index : 83u32 , item_array_index : 85u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 90u32 , parent_index : 85u32 , item_array_index : 86u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 92u32 , parent_index : 85u32 , item_array_index : 87u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 97u32 , parent_index : 85u32 , item_array_index : 88u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 91u32 , parent_index : 87u32 , item_array_index : 89u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 92u32 , parent_index : 90u32 , item_array_index : 90u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 93u32 , parent_index : 88u32 , item_array_index : 91u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 96u32 , parent_index : 92u32 , item_array_index : 92u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 96u32 , parent_index : 92u32 , item_array_index : 93u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 97u32 , parent_index : 92u32 , item_array_index : 94u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 97u32 , parent_index : 94u32 , item_array_index : 95u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 98u32 , parent_index : 89u32 , item_array_index : 96u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 101u32 , parent_index : 97u32 , item_array_index : 97u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 101u32 , parent_index : 97u32 , item_array_index : 98u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 102u32 , parent_index : 97u32 , item_array_index : 99u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 102u32 , parent_index : 99u32 , item_array_index : 100u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 104u32 , parent_index : 55u32 , item_array_index : 101u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 104u32 , parent_index : 55u32 , item_array_index : 102u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             103usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_3 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_5 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_14 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_6 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_9 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_11 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#_opacity_46 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_background_48 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_scroll_view_49 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#rectangle_47 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_59 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_viewport_51 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_border_55 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_opacity_56 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_57 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_60 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_border_61 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_opacity_62 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#b_config_8 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_63 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#_opacity_46 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_background_48 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_scroll_view_49 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#rectangle_47 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_59 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_viewport_51 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_border_55 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_opacity_56 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_57 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_60 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_border_61 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_opacity_62 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#a_config_13 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_63 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#rectangle_15 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_17 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_42 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_16 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_18 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_38 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_19 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#res_line_20 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_21 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_26 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_32 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_viewport_22 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_27 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_border_28 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_opacity_29 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_31 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_30 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_33 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_border_34 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_opacity_35 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_37 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_36 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#root_45 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#root_65 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#txt_66 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_40 }
             + {
                 * & InnerButton_root_65 :: FIELD_OFFSETS . r#touch_67 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#_opacity_46 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_background_48 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_scroll_view_49 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#rectangle_47 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_50 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_visibility_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_59 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_flickable_viewport_51 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_text_input_52 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_vertical_bar_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_border_55 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_opacity_56 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_58 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_57 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_horizontal_bar_60 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_border_61 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_opacity_62 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_touch_area_64 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#res_check_41 }
             + {
                 * & InnerTextEdit_root_45 :: FIELD_OFFSETS . r#i_thumb_63 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_43 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#total_lines_44 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self ,) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter_ . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let self_rc = sp :: VRcMapped :: origin (& self . self_weak . get () . unwrap () . upgrade () . unwrap () ,) ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& self_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter_ . get () . cloned () }
         }
     impl sp :: PinnedDrop for InnerAppWindow {
         fn drop (self : core :: pin :: Pin < & mut InnerAppWindow >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ItemTreeVTable_static ! (static VT for self :: InnerAppWindow) ;
             new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAppWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAppWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AppWindow (sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) ;
     impl r#AppWindow {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . global_ColorSchemeSelector_68 . clone () . init (& inner) ;
             inner . globals . global_CupertinoPalette_69 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_70 . clone () . init (& inner) ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_lines (& self) -> sp :: ModelRc < r#Lines > {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_lines }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_lines (& self , value : sp :: ModelRc < r#Lines >) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_lines }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_refresh (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_refresh }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_refresh (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_refresh }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_res (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_res }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_res (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_res }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_res_backup (& self) -> sp :: ModelRc < sp :: SharedString > {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_res_backup }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_res_backup (& self , value : sp :: ModelRc < sp :: SharedString >) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_res_backup }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_select (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_select }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_select (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_select }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn invoke_summit (& self , arg_0 : sp :: SharedString , arg_1 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_summit }
            ) . apply_pin (_self) . call (& (arg_0 , arg_1 ,)) }
         # [allow (dead_code)] pub fn on_summit (& self , mut f : impl FnMut (sp :: SharedString , sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_summit }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone ())) }
         # [allow (dead_code)] pub fn get_t_lines (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_t_lines }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_t_lines (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_t_lines }
            ) . apply_pin (_self) . set (value as _) }
         }
     impl From < r#AppWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > {
         fn from (value : r#AppWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#AppWindow {
         type Inner = InnerAppWindow ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_AppWindow {
         global_ColorSchemeSelector_68 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_68 >> , global_CupertinoPalette_69 : :: core :: pin :: Pin < sp :: Rc < InnerCupertinoPalette_69 >> , global_StyleMetrics_70 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_70 >> , }
     impl :: core :: default :: Default for Globals_AppWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_68 : InnerColorSchemeSelector_68 :: new () , global_CupertinoPalette_69 : InnerCupertinoPalette_69 :: new () , global_StyleMetrics_70 : InnerStyleMetrics_70 :: new () , }
             }
         }
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_4_1 = slint :: VersionCheck_1_4_1 ;
     }
 # [allow (unused_imports)] pub use slint_generatedAppWindow :: {
     r#AppWindow , r#Lines , r#TextStyle }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
