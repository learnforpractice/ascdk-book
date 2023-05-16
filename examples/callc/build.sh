cdt-cc -c -o say_hello.o say_hello.c
python-contract build --linker-flags="say_hello.o" test.codon
