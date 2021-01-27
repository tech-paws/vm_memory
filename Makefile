BUILDDIR = build
LDFLAGS =
CXX = clang++
CXXFLAGS = -I. -Isrc/ -Ipublic/cpp/ -Wall -std=c++17 -g3

UNAME_S := $(shell uname -s)

SOURCES := $(shell find src -type f -name '*.cpp' -maxdepth 1)

ifeq ($(UNAME_S),Linux)
	SOURCES += src/platform/linux.cpp
endif

OBJECTS := $(addprefix $(BUILDDIR)/,$(SOURCES:%.cpp=%.o))

LIBRARY = libvm_memory.so

build: $(LIBRARY)

$(LIBRARY): $(OBJECTS)
	$(CXX) -shared $(LDFLAGS) $(OBJECTS) -o build/$(LIBRARY)
	bindgen public/cpp/vm_memory.hpp -o public/rust/vm_memory/src/c_api.rs

$(BUILDDIR)/%.o: %.cpp
	mkdir -p $(BUILDDIR)/$(dir $<)
	$(CXX) $(CXXFLAGS) $(IMGUI_FLAGS) -c $< -o $@

clean:
	rm -rf $(BUILDDIR)
