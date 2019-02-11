   @20   // MAIN 縦
   D=A
   @counter_v
   M=D

   @SCREEN
   D=A
   @address
   M=D
(LOOP_V)
   @256
   D=A
   @counter_h
   M=D
(LOOP_H)

   @7  // 色
   D=A
   @address
   A=M
   M=D

   @address // address++
   D=M
   @1
   D=D+A
   @address
   M=D

   @counter_h
   MD=M-1
   @LOOP_H
   D;JGT
   
   @counter_v
   MD=M-1
   @LOOP_V
   D;JGT
(INFINITE_LOOP)
   @INFINITE_LOOP
   0;JMP
