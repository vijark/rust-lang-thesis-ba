# Makefile for C program

TARGET   = target
OBJDIR   = $(TARGET)/objdir
OBJ      = $(addprefix $(OBJDIR)/, $(patsubst %.c, %.o, $(wildcard *.c)))
NAME     = $(TARGET)/app
GCC	     = gcc
CPPFLAGS = -Wall

all: $(OBJDIR) $(NAME)

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(OBJDIR)/%.o: %.c
	$(GCC) $(CPPFLAGS) -c $< -o $@

$(NAME): $(OBJ)
	$(GCC) $(LDFLAGS) -o $@ $^

clean:
	@rm -rf $(TARGET)
