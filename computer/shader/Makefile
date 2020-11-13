glsl = $(wildcard ./*.vert ./*.frag)
spirv = $(addsuffix .spv, $(glsl))

.PHONY: all clean
all: $(spirv)

$(spirv): %.spv: %
	glslangValidator -V $< -o $@

clean:
	rm -r $(spirv)
