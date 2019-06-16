# Makefile for C++ program

TARGET   = target
OBJDIR   = $(TARGET)/objdir
OBJ      = $(addprefix $(OBJDIR)/, $(patsubst %.cpp, %.o, $(wildcard *.cpp)))
NAME     = $(TARGET)/app
COMPILER = g++
CPPFLAGS = -Wall

all: $(OBJDIR) $(NAME)

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(OBJDIR)/%.o: %.cpp
	$(COMPILER) $(CPPFLAGS) -c $< -o $@

$(NAME): $(OBJ)
	$(COMPILER) $(LDFLAGS) -o $@ $^

clean:
	@rm -rf $(TARGET)
