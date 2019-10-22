add_executable(bin main.cpp)

#################### Import the 'io_tools' library here ########################
target_link_libraries(bin ${LIB_IO_TOOLS})

#################### Create a library 'image' below this line ##################
add_library(LIB_IMAGE STATIC image.cpp)
target_link_libraries(bin ${LIB_IMAGE})
