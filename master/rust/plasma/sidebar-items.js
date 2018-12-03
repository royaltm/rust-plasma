initSidebarItems({"fn":[["render_part","Renders the part of the plasma into the provided `buffer` without the [Plasma] instance."]],"struct":[["F32PaPairIterator",""],["PhaseAmp","Holds a phase and an amplitude along with their animation state."],["PhaseAmpCfg","Holds parameters of phase and amplitude changes for [PhaseAmpConfig] trait."],["PhaseAmpsPairIterator",""],["PixelBufRGB24","A [PixelBuffer] tool for a RGB24 buffer (3 bytes/pixel: red, green, blue)."],["PixelBufRGBA8","A [PixelBuffer] tool for a RGBA8 buffer (4 bytes/pixel: red, green, blue, alpha)."],["PixelRgb","A struct representing one or more pixels in the linear RGB color space."],["Plasma","The struct that holds the meta information about current plasma state"],["PlasmaInterCalcProducer","Provides a default implementation of a [IntermediateCalculatorProducer]."],["PlasmaLineCalc","Provides a default implementation of a [IntermediateCalculator]."],["PlasmaMixIter","Provides a default implementation of an iterator of [PlasmaLineCalc]."],["PlasmaMixer","A default implementation of a [Mixer] is provided for this struct."],["RgbIter","An iterator of [PixelRgb] color components."],["RgbaIter","An iterator of [PixelRgb] color components plus an alpha component."]],"trait":[["ICProducer","A convenient trait alias for plasma render methods."],["IntermediateCalculator","Implementations of this trait should compute the vertical and horizontal intermediate data for a [Mixer]."],["IntermediateCalculatorProducer","Implementations of this trait should produce an iterator of an [IntermediateCalculator] tool."],["Mixer","Implementations of this trait should compute the color of each pixel based on an intermediate data created by a [IntermediateCalculator]."],["PhaseAmpAccess","A trait for querying and updating phase'n'amplitude"],["PhaseAmpConfig","A trait for querying parameters of phase and amplitude changes."],["PhaseAmpDataExp","A trait that allows importing and exporting of phase'n'amplitude data"],["PhaseAmpsSelect","A trait that allows selecting a subset of phase'n'amplitude and iterate over pairs of it."],["PixelBuffer","The trait for putting pixels into byte buffers."],["ToColor8","Provides a method of converting color part from a `f32` type to a `u8`."]],"type":[["PlasmaICP","A convenient type to be used with [Plasma.render]."],["Xf32","All the intermediate calculations are performed on this type."]]});