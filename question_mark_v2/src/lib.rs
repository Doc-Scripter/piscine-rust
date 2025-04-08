pub struct One {
 pub   first_layer: Option<Two>,
}
pub struct Two {
    pub second_layer: Option<Three>,
}
pub struct Three {
    pub third_layer: Option<Four>,
}
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        match self.first_layer{
            Some(first_layer) => {
                match first_layer.second_layer{
                    Some(second_layer) => {
                        match second_layer.third_layer{
                            Some(third_layer) => {
                                match third_layer.fourth_layer{
                                    Some(fourth_layer) => Some(fourth_layer),
                                    none => none
                                }
                            },
                            None => None
                        }
                    },
                    None => None
                }
            },
            None => None
        }
    }
}