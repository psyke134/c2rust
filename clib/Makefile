INCLUDE_DIR := include
SRC_DIR := src
BIN_DIR := bin
LIB_DIR := lib

CC := gcc
CFLAGS := -Wall -Wextra -I$(INCLUDE_DIR)

LIB_SOURCES := $(wildcard $(SRC_DIR)/*.c)
LIB_OBJECTS := $(patsubst $(SRC_DIR)/%.c,$(BIN_DIR)/%.o,$(LIB_SOURCES))
LIB_NAME := $(LIB_DIR)/libauto.so

MAIN_SOURCE := main.c
MAIN_EXE := main

.PHONY: lib debug

debug:
	@echo $(LIB_OBJECTS)

lib: $(LIB_OBJECTS)
	$(CC) -shared -o $(LIB_NAME) $^

main: $(LIB_NAME)
	$(CC) $(CFLAGS) -o ${MAIN_EXE} ${MAIN_SOURCE} -L ${LIB_DIR} -lauto

$(BIN_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) -fPIC -c $< -o $@