Files generated by roc glue will appear here. Do not edit them manually and/or commit to git!

**Luke's comment** I think you want to commit these to git so that you have working code/tests at all times
Agree, tests working is critical! My idea was to have a top-level build files controlled with cargo such that "cargo test" would build all necessary dependencies. Given how quickly roc compiler evolves, having bits of it copied here is ill advised (i've had such bad experiences with Zig at least, where having any hard-references to compiler internals is a very bd idea tm).
