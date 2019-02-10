//   @16   // MAIN 縦16px
   @512   // MAIN 縦
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

//   @65535  // 白
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

//   @624  // 640 - 16(LOOP_H) 
//   D=D+A
//   @address
//   M=D
   
   @counter_v
   MD=M-1
   @LOOP_V
   D;JGT
(INFINITE_LOOP)
   @INFINITE_LOOP
   0;JMP
