# generates our dynamic library plugin

roc check plugin_logic.roc  || ( echo "Please fix your code!" && exit 1 ) # prevents compiler from crashing when code is really bad 
roc build --lib plugin_logic.roc
