CXX      = g++
CFLAGS   = -Wall
CPPFLAGS = $(CFLAGS) -Irabbitmq-c/librabbitmq -I/usr/local/include -Lrabbitmq-c/build/librabbitmq -L/usr/local/lib -Iinclude/

LIBRARIES= rabbitmq ssl crypto
LIBS     = $(addprefix -l,$(LIBRARIES))

export LIBNAME  = amqpcpp
LIBFILE  = lib$(LIBNAME).a
LIBSO    = lib$(LIBNAME).so

SOURCES  = src/AMQP.cpp src/AMQPBase.cpp src/AMQPException.cpp src/AMQPMessage.cpp src/AMQPExchange.cpp src/AMQPQueue.cpp

OBJECTS  = $(SOURCES:.cpp=.o)

all: lib 

lib: $(LIBFILE) $(LIBSO)

$(LIBFILE): $(OBJECTS)
	$(AR) rcs $@ $(OBJECTS)

$(LIBSO): $(OBJECTS)
	$(CXX) $(CPPFLAGS) -fPIC -shared $(SOURCES) -o $(LIBSO)
	

install:
	cp $(LIBSO) /usr/lib
	ldconfig
	cp include/* /usr/include/
	
uninstall:
	rm /usr/local/lib/$(LIBSO)
	rm /usr/local/include/AMQPcpp.h
	
clean:
	rm -f $(OBJECTS) $(LIBFILE) $(LIBSO)
