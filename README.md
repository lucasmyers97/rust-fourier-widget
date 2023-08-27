# Rust Fourier Widget

Construct a Fourier series with this widget by using sliders to specify the Fourier coefficients.
Watch the partial series converge to a function (which you can input) by minimizing the $L_2$ norm at each step in the partial series.
This was originally written in Javascript, but I rewrote in rust to get some practice with rust and immediate-mode GUIs.

# Todo

- [ ] Refactor to make each component its own function.
- [ ] Refactor things out to their own modules (need to learn rust modules).
- [ ] Make sliders fit the width properly.
- [ ] Detect when screen width is too small and create dropdown menu (or switch) which goes between sin coefficients and cos coefficients
- [ ] Put label to the left of sliders.
- [ ] Make function input box smaller.
- [ ] Center function input box.
- [ ] Put slider value outside of slider limits textboxes.
- [ ] Profile to see what, if anything, is slow
- [ ] Publish with WebGPU.
