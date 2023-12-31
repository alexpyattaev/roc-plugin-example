# generates our dynamic library plugin


roc check plugin_logic.roc  || ( echo "Please fix your code!" && exit 1 ) # prevents compiler from crashing when code is really bad 

# build unlinked version of the library 
roc build --no-link plugin_logic.roc
# use ar to make an archive that rust linker can eat...
ar rc platform_wrapper/libapp.a plugin_logic.o

# build shared library version as well since we might want it eventually
roc build --lib plugin_logic.roc
