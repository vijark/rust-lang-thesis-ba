# Makefile for C++ program

TARGET   = target
OBJDIR   = $(TARGET)/objdir
OBJ      = $(addprefix $(OBJDIR)/, $(patsubst %.cpp, %.o, $(wildcard *.cpp)))
NAME     = $(TARGET)/app
GCC	     = g++
CPPFLAGS = -Wall

all: $(OBJDIR) $(NAME)

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(OBJDIR)/%.o: %.cpp
	$(GCC) $(CPPFLAGS) -c $< -o $@

$(NAME): $(OBJ)
	$(GCC) $(LDFLAGS) -o $@ $^

clean:
	@rm -rf $(TARGET)
