var N = null;var searchIndex = {};
searchIndex["plasma"]={"doc":"A [Plasma] struct and tools for rendering animated eye-candy pixels.","items":[[3,"PixelRgb","plasma","A struct representing one or more pixels in the linear RGB color space.",N,N],[12,"r","","",0,N],[12,"g","","",0,N],[12,"b","","",0,N],[3,"RgbIter","","An iterator of [PixelRgb] color components.",N,N],[3,"RgbaIter","","An iterator of [PixelRgb] color components plus an alpha component.",N,N],[3,"PlasmaMixer","","A default implementation of a [Mixer] is provided for this struct.",N,N],[3,"PlasmaLineCalc","","Provides a default implementation of a [IntermediateCalculator].",N,N],[3,"PlasmaMixIter","","Provides a default implementation of an iterator of [PlasmaLineCalc].",N,N],[3,"PlasmaLineCalcProducer","","Provides a default implementation of a [IntermediateCalculatorProducer].",N,N],[3,"PhaseAmpCfg","","Holds parameters of phase and amplitude changes for [PhaseAmpConfig] trait.",N,N],[3,"PhaseAmp","","Holds a phase and an amplitude along with their animation state.",N,N],[3,"PhaseAmpsPairIterator","","",N,N],[3,"F32PaPairIterator","","",N,N],[3,"PixelBufRGB24","","A [PixelBuffer] tool for a RGB24 buffer (3 bytes/pixel: red, green, blue).",N,N],[3,"PixelBufRGBA8","","A [PixelBuffer] tool for a RGBA8 buffer (4 bytes/pixel: red, green, blue, alpha).",N,N],[3,"Plasma","","The struct that holds the meta information about current plasma state",N,N],[12,"pixel_width","","The plasma pixel width",1,N],[12,"pixel_height","","The plasma pixel height",1,N],[5,"render_part","","Renders the part of the plasma into the provided `buffer` without the [Plasma] instance.",N,N],[11,"new","","Creates an instance of [PixelRgb] from RGB color components.",0,[[["f32"],["f32"],["f32"]],["pixelrgb"]]],[11,"iter_rgb_values","","Creates a [RgbIter] from this instance of [PixelRgb].",0,[[["self"]],["rgbiter"]]],[11,"iter_rgba_values","","Creates a [RgbaIter] from this instance of [PixelRgb].",0,[[["self"],["f32"]],["rgbaiter"]]],[11,"from_hsv","","Creates an instance of a [PixelRgb] from HSV color components.",0,[[["f32"],["f32"],["f32"]],["pixelrgb"]]],[11,"new","","",2,[[],["self"]]],[11,"new","","Creates new [PhaseAmpCfg] instance from the provided arguments.",3,[[["f32"],["f32"]],["self"]]],[11,"new","","Creates randomized single phase and amplitude pair.",4,[[["c"],["r"]],["self"]]],[11,"update","","Performs a one step update of the phase and amplitude pair animation.",4,[[["self"],["c"],["r"]]]],[11,"new","","Creates new plasma instance.",1,[[["u32"],["u32"],["phaseampcfg"],["r"]],["self"]]],[11,"update","","Animates the plasma by modifying the internal [PhaseAmp] variables.",1,[[["self"],["r"]]]],[11,"render","","Renders the plasma into the provided `buffer`.",1,N],[11,"render_part","","Renders the part of the plasma into the provided `buffer`.",1,N],[11,"import_phase_amps","","Import the internal plasma state from a slice of 32bit floats.",1,N],[11,"export_phase_amps","","Exports the internal plasma state into the [Vec] of 32bit floats.",1,[[["self"],["vec"]]]],[11,"min_steps","","",1,[[["self"]],["f32"]]],[11,"max_steps","","",1,[[["self"]],["f32"]]],[8,"IntermediateCalculator","","Implementations of this trait should compute the vertical and horizontal intermediate data for a [Mixer].",N,N],[10,"calculate","","Computes an intermediate data for a given angle.",5,[[["self"],["t"]],["t"]]],[8,"Mixer","","Implementations of this trait should compute the color of each pixel based on an intermediate data created by a [IntermediateCalculator].",N,N],[16,"IntermediateH","","This type should be an array of the type T for an intermediate horizontal data.",6,N],[16,"IntermediateV","","This type should be an array of the type T for an intermediate vertical data.",6,N],[11,"intermediate_h_len","","Returns the number of intermediate horizontal values.",6,[[],["usize"]]],[11,"intermediate_v_len","","Returns the number of intermediate vertical values.",6,[[],["usize"]]],[10,"mix_pixels","","The implementors should compute a pixel and send it as an instance of [PixelRgb] to the provided `next_pixel` function.",6,N],[8,"IntermediateCalculatorProducer","","Implementations of this trait should produce an iterator of an [IntermediateCalculator] tool.",N,N],[16,"CalcIterH","","Provide an iterator implementation which produce [IntermediateCalculator] tools. The iterator must be a [ExactSizeIterator] with exactly the same length as the associated [Mixer::IntermediateH] array's number of elements.",7,N],[16,"CalcIterV","","Provide an iterator implementation which produce [IntermediateCalculator] tools. The iterator must be a [ExactSizeIterator] with exactly the same length as the associated [Mixer::IntermediateV] array's number of elements.",7,N],[16,"LineCalcH","","Provide an implementation of a [IntermediateCalculator] for horizontal intermediate data.",7,N],[16,"LineCalcV","","Provide an implementation of a [IntermediateCalculator] for vertical intermediate data.",7,N],[10,"compose_h_iter","","Should return an instance of a [IntermediateCalculatorProducer::LineCalcH]. The input data references an implementation of [PhaseAmpsSelect] tool.",7,N],[10,"compose_v_iter","","Should return an instance of a [IntermediateCalculatorProducer::LineCalcV]. The input data references an implementation of [PhaseAmpsSelect] tool.",7,N],[8,"PhaseAmpConfig","","A trait for querying parameters of phase and amplitude changes.",N,N],[10,"min_steps","","",8,[[["self"]],["f32"]]],[10,"max_steps","","",8,[[["self"]],["f32"]]],[10,"delta_phase_abs_max","","",8,[[["self"]],["f32"]]],[10,"delta_delta_phase_abs_max","","",8,[[["self"]],["f32"]]],[8,"PhaseAmpAccess","","A trait for querying and updating phase'n'amplitude",N,N],[10,"phase","","",9,[[["self"]],["f32"]]],[10,"amplitude","","",9,[[["self"]],["f32"]]],[10,"set_phase","","",9,[[["self"],["f32"]]]],[10,"set_amplitude","","",9,[[["self"],["f32"]]]],[11,"export","","",9,[[["self"],["vec"]]]],[8,"PhaseAmpDataExp","","A trait that allows importing and exporting of phase'n'amplitude data",N,N],[10,"export_phase_amps","","",10,[[["self"],["vec"]]]],[10,"import_phase_amps","","",10,N],[8,"PhaseAmpsSelect","","A trait that allows selecting a subset of phase'n'amplitude and iterate over pairs of it.",N,N],[16,"PairIter","","",11,N],[16,"Item","","",11,N],[10,"select","","The range should always be bounded. # Panics",11,[[["self"],["range",["usize"]]],["self"]]],[10,"into_pa_pair_iter","","",11,N],[8,"PixelBuffer","","The trait for putting pixels into byte buffers.",N,N],[18,"PIXEL_BYTES","","Specifies how many bytes a single pixel occupies.",12,N],[10,"put_pixel","","Puts bytes from a `pixel` into the provided `buffer` using a provided writer.",12,[[["i"],["pixelrgb"]]]],[8,"ToColor8","","Provides a method of converting color part from a `f32` type to a `u8`.",N,N],[10,"to_color_u8clamped","","",13,[[["self"]],["u8"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"from","","",0,[[["t"]],["t"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"into","","",14,[[["self"]],["u"]]],[11,"to_owned","","",14,[[["self"]],["t"]]],[11,"clone_into","","",14,N],[11,"from","","",14,[[["t"]],["t"]]],[11,"into_iter","","",14,[[["self"]],["i"]]],[11,"try_from","","",14,[[["u"]],["result"]]],[11,"borrow","","",14,[[["self"]],["t"]]],[11,"get_type_id","","",14,[[["self"]],["typeid"]]],[11,"borrow_mut","","",14,[[["self"]],["t"]]],[11,"try_into","","",14,[[["self"]],["result"]]],[11,"into","","",15,[[["self"]],["u"]]],[11,"to_owned","","",15,[[["self"]],["t"]]],[11,"clone_into","","",15,N],[11,"from","","",15,[[["t"]],["t"]]],[11,"into_iter","","",15,[[["self"]],["i"]]],[11,"try_from","","",15,[[["u"]],["result"]]],[11,"borrow","","",15,[[["self"]],["t"]]],[11,"get_type_id","","",15,[[["self"]],["typeid"]]],[11,"borrow_mut","","",15,[[["self"]],["t"]]],[11,"try_into","","",15,[[["self"]],["result"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,N],[11,"from","","",2,[[["t"]],["t"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"into","","",16,[[["self"]],["u"]]],[11,"from","","",16,[[["t"]],["t"]]],[11,"try_from","","",16,[[["u"]],["result"]]],[11,"borrow","","",16,[[["self"]],["t"]]],[11,"get_type_id","","",16,[[["self"]],["typeid"]]],[11,"borrow_mut","","",16,[[["self"]],["t"]]],[11,"try_into","","",16,[[["self"]],["result"]]],[11,"into","","",17,[[["self"]],["u"]]],[11,"from","","",17,[[["t"]],["t"]]],[11,"into_iter","","",17,[[["self"]],["i"]]],[11,"try_from","","",17,[[["u"]],["result"]]],[11,"borrow","","",17,[[["self"]],["t"]]],[11,"get_type_id","","",17,[[["self"]],["typeid"]]],[11,"borrow_mut","","",17,[[["self"]],["t"]]],[11,"try_into","","",17,[[["self"]],["result"]]],[11,"into","","",18,[[["self"]],["u"]]],[11,"from","","",18,[[["t"]],["t"]]],[11,"try_from","","",18,[[["u"]],["result"]]],[11,"borrow","","",18,[[["self"]],["t"]]],[11,"get_type_id","","",18,[[["self"]],["typeid"]]],[11,"borrow_mut","","",18,[[["self"]],["t"]]],[11,"try_into","","",18,[[["self"]],["result"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,N],[11,"from","","",3,[[["t"]],["t"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"into","","",4,[[["self"]],["u"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,N],[11,"from","","",4,[[["t"]],["t"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"into","","",19,[[["self"]],["u"]]],[11,"from","","",19,[[["t"]],["t"]]],[11,"into_iter","","",19,[[["self"]],["i"]]],[11,"try_from","","",19,[[["u"]],["result"]]],[11,"borrow","","",19,[[["self"]],["t"]]],[11,"get_type_id","","",19,[[["self"]],["typeid"]]],[11,"borrow_mut","","",19,[[["self"]],["t"]]],[11,"try_into","","",19,[[["self"]],["result"]]],[11,"into","","",20,[[["self"]],["u"]]],[11,"from","","",20,[[["t"]],["t"]]],[11,"into_iter","","",20,[[["self"]],["i"]]],[11,"try_from","","",20,[[["u"]],["result"]]],[11,"borrow","","",20,[[["self"]],["t"]]],[11,"get_type_id","","",20,[[["self"]],["typeid"]]],[11,"borrow_mut","","",20,[[["self"]],["t"]]],[11,"try_into","","",20,[[["self"]],["result"]]],[11,"into","","",21,[[["self"]],["u"]]],[11,"from","","",21,[[["t"]],["t"]]],[11,"try_from","","",21,[[["u"]],["result"]]],[11,"borrow","","",21,[[["self"]],["t"]]],[11,"get_type_id","","",21,[[["self"]],["typeid"]]],[11,"borrow_mut","","",21,[[["self"]],["t"]]],[11,"try_into","","",21,[[["self"]],["result"]]],[11,"into","","",22,[[["self"]],["u"]]],[11,"from","","",22,[[["t"]],["t"]]],[11,"try_from","","",22,[[["u"]],["result"]]],[11,"borrow","","",22,[[["self"]],["t"]]],[11,"get_type_id","","",22,[[["self"]],["typeid"]]],[11,"borrow_mut","","",22,[[["self"]],["t"]]],[11,"try_into","","",22,[[["self"]],["result"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,N],[11,"from","","",1,[[["t"]],["t"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"calculate","","",16,[[["self"],["f32"]],["f32"]]],[11,"mix_pixels","","",2,N],[11,"compose_h_iter","","",18,N],[11,"compose_v_iter","","",18,N],[11,"min_steps","","",3,[[["self"]],["f32"]]],[11,"max_steps","","",3,[[["self"]],["f32"]]],[11,"delta_phase_abs_max","","",3,[[["self"]],["f32"]]],[11,"delta_delta_phase_abs_max","","",3,[[["self"]],["f32"]]],[11,"phase","","",4,[[["self"]],["f32"]]],[11,"set_phase","","",4,[[["self"],["f32"]]]],[11,"amplitude","","",4,[[["self"]],["f32"]]],[11,"set_amplitude","","",4,[[["self"],["f32"]]]],[11,"put_pixel","","",21,[[["i"],["pixelrgb"]]]],[11,"put_pixel","","",22,[[["i"],["pixelrgb"]]]],[11,"next","","",14,[[["self"]],["option",["f32"]]]],[11,"size_hint","","",14,N],[11,"next","","",15,[[["self"]],["option",["f32"]]]],[11,"size_hint","","",15,N],[11,"next","","",17,[[["self"]],["option"]]],[11,"next","","",19,[[["self"]],["option"]]],[11,"next","","",20,[[["self"]],["option"]]],[11,"len","","",14,[[["self"]],["usize"]]],[11,"len","","",15,[[["self"]],["usize"]]],[11,"len","","",17,[[["self"]],["usize"]]],[11,"len","","",19,[[["self"]],["usize"]]],[11,"len","","",20,[[["self"]],["usize"]]],[11,"default","","",0,[[],["pixelrgb"]]],[11,"default","","",4,[[],["phaseamp"]]],[11,"default","","",3,[[],["self"]]],[11,"eq","","",0,[[["self"],["pixelrgb"]],["bool"]]],[11,"ne","","",0,[[["self"],["pixelrgb"]],["bool"]]],[11,"eq","","",2,[[["self"],["plasmamixer"]],["bool"]]],[11,"ne","","",2,[[["self"],["plasmamixer"]],["bool"]]],[11,"eq","","",3,[[["self"],["phaseampcfg"]],["bool"]]],[11,"ne","","",3,[[["self"],["phaseampcfg"]],["bool"]]],[11,"eq","","",4,[[["self"],["phaseamp"]],["bool"]]],[11,"ne","","",4,[[["self"],["phaseamp"]],["bool"]]],[11,"eq","","",1,[[["self"],["plasma"]],["bool"]]],[11,"ne","","",1,[[["self"],["plasma"]],["bool"]]],[11,"clone","","",0,[[["self"]],["pixelrgb"]]],[11,"clone","","",14,[[["self"]],["rgbiter"]]],[11,"clone","","",15,[[["self"]],["rgbaiter"]]],[11,"clone","","",2,[[["self"]],["plasmamixer"]]],[11,"clone","","",3,[[["self"]],["phaseampcfg"]]],[11,"clone","","",4,[[["self"]],["phaseamp"]]],[11,"clone","","",1,[[["self"]],["plasma"]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",2,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"add","","",0,[[["self"],["pixelrgb"]],["pixelrgb"]]],[11,"sub","","",0,[[["self"],["pixelrgb"]],["pixelrgb"]]],[11,"mul","","",0,[[["self"],["__rhst"]],["pixelrgb"]]],[11,"div","","",0,[[["self"],["__rhst"]],["pixelrgb"]]],[11,"rem","","",0,[[["self"],["__rhst"]],["pixelrgb"]]],[11,"neg","","",0,[[["self"]],["pixelrgb"]]],[11,"add_assign","","",0,[[["self"],["pixelrgb"]]]],[11,"sub_assign","","",0,[[["self"],["pixelrgb"]]]],[11,"mul_assign","","",0,[[["self"],["__rhst"]]]],[11,"div_assign","","",0,[[["self"],["__rhst"]]]],[11,"rem_assign","","",0,[[["self"],["__rhst"]]]],[11,"intermediate_h_len","","Returns the number of intermediate horizontal values.",6,[[],["usize"]]],[11,"intermediate_v_len","","Returns the number of intermediate vertical values.",6,[[],["usize"]]],[11,"export","","",9,[[["self"],["vec"]]]],[18,"PIXEL_BYTES","","Specifies how many bytes a single pixel occupies.",12,N]],"paths":[[3,"PixelRgb"],[3,"Plasma"],[3,"PlasmaMixer"],[3,"PhaseAmpCfg"],[3,"PhaseAmp"],[8,"IntermediateCalculator"],[8,"Mixer"],[8,"IntermediateCalculatorProducer"],[8,"PhaseAmpConfig"],[8,"PhaseAmpAccess"],[8,"PhaseAmpDataExp"],[8,"PhaseAmpsSelect"],[8,"PixelBuffer"],[8,"ToColor8"],[3,"RgbIter"],[3,"RgbaIter"],[3,"PlasmaLineCalc"],[3,"PlasmaMixIter"],[3,"PlasmaLineCalcProducer"],[3,"PhaseAmpsPairIterator"],[3,"F32PaPairIterator"],[3,"PixelBufRGB24"],[3,"PixelBufRGBA8"]]};
initSearch(searchIndex);
