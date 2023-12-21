
# Note the game developer will need to have roc repository cloned on their 
# system to (re)generate the glue types.
#
# For users of the Roc platform this wont be needed as the game developer should
# package that into a URL with the artefacts compiled for each supported distribution/architecture
roc glue ../roc/crates/glue/src/RustGlue.roc generated_glue platform/main.roc