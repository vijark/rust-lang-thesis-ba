# Makefile for C program

TARGET   = target
OBJDIR   = $(TARGET)/objdir
OBJ      = $(addprefix $(OBJDIR)/, $(patsubst %.c, %.o, $(wildcard *.c)))
NAME     = $(TARGET)/app
COMPILER = gcc
CPPFLAGS = -Wall
LDFLAGS  = -lpthread

all: $(OBJDIR) $(NAME)

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(OBJDIR)/%.o: %.c
	$(COMPILER) $(CPPFLAGS) -c $< -o $@

$(NAME): $(OBJ)
	$(COMPILER) $(LDFLAGS) -o $@ $^

clean:
	@rm -rf $(TARGET)
