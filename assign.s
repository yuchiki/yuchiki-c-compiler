.intel_syntax noprefix
  push rbp
  mov rbp, rsp
  sub rsp, 8
  call main
  add rsp, 8
  mov rsp, rbp
  pop rbp
  ret
.globl main
main:
  push rbp
  mov rbp, rsp
  sub rsp, 12
  mov rax, rbp
  sub rax, 8
  push rax
  push 3
  pop edi
  pop rax
  mov [rax], edi
  push edi
  pop rax
  mov rax, rbp
  sub rax, 8
  push rax
  pop rax
  mov eax, [rax]
  push eax
  pop rax
  mov rsp, rbp
  pop rbp
  ret
  mov rsp, rbp
  pop rbp
  ret
