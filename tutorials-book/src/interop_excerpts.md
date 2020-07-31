Although ArrayFire is quite extensive, there remain many cases in which you may want to write custom
kernels in CUDA or OpenCL. For example, you may wish to add ArrayFire to an existing code base to
increase your productivity, or you may need to supplement ArrayFire's functionality with your own
custom implementation of specific algorithms.

It is fairly straightforward to interface ArrayFire with your own custom code. ArrayFire provides
several functions to ease this process including:

Adding ArrayFire to an existing application is slightly more involved and can be somewhat tricky due
to several optimizations we implement. The most important are as follows:

- ArrayFire assumes control of all memory provided to it.
- ArrayFire does not (in general) support in-place memory transactions.

We will discuss the implications of these items below. To add ArrayFire to existing code you need to:
