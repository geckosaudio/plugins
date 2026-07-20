use truce::prelude::*;
use truce_gui_types::layout::{GridLayout, knob, widgets};

#[derive(Params)]
pub struct GeckosAudioTunerParams {
    #[param(
        name = "Gain",
        range = "linear(-60, 6)",
        unit = "dB",
        smooth = "exp(5)"
    )]
    pub gain: FloatParam,
}

use GeckosAudioTunerParamsParamId as P;

#[derive(Default)]
pub struct GeckosAudioTuner {
}

impl PluginLogic for GeckosAudioTuner {
    type Params = GeckosAudioTunerParams;
    type DspState = Self;

    fn process(
        _state: &mut Self::DspState,
        params: &Self::Params,
        buffer: &mut AudioBuffer,
        _events: &EventList,
        _context: &mut ProcessContext,
    ) -> ProcessStatus {
        for i in 0..buffer.num_samples() {
            let gain = db_to_linear(params.gain.read());
            for ch in 0..buffer.channels() {
                let (inp, out) = buffer.io(ch);
                out[i] = inp[i] * gain;
            }
        }
        ProcessStatus::Normal
    }

    fn editor(params: Arc<GeckosAudioTunerParams>) -> Box<dyn Editor> {
        truce_gui::default_editor(
            params,
            GridLayout::build(vec![widgets(vec![knob(P::Gain, "Gain")])]),
        )
    }
}

truce::plugin! {
    logic: GeckosAudioTuner,
    params: GeckosAudioTunerParams,
}

truce::enable_rt_paranoid!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passthrough() {
        use std::time::Duration;
        use truce_test::{InputSource, assertions, driver};

        let result = driver!(Plugin)
            .duration(Duration::from_millis(100))
            .input(InputSource::Constant(0.5))
            .run();
        assertions::assert_nonzero(&result);
        assertions::assert_no_nans(&result);
        assertions::assert_peak_below(&result, 1.0);
    }
}
