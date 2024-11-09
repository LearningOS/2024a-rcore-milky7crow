### 实现的功能

- 在`TaskControlBlock`中添加`syscall_times`和`start_time`两个字段，分别记录任务的系统调用次数和首次被调度时间。
- 任务首次被调度时（判断开始时间为0），设置首次被调度时间为当前时间。
- `TaskManager`提供公共方法获取当前任务的`syscall_times`和`start_time`，提供公共方法增加（计数器计数）`syscall_times`。
- 在系统调用分发函数`syscall`中，根据`syscall_id`信息设置当前任务的系统调用计数器。
- 实现`sys_task_info`系统调用，根据从`TASK_MANAGER`获取的当前任务信息构造/修改`TaskInfo`。

### 简答作业

#### 第一题

使用的sbi版本：`0.2.0-alpha.2`。

- `ch2b_bad_address.rs`：向无效地址（此处为0）写入数据。
- `ch2b_bad_instructions.rs`：在U态使用S态特权指令`sret`。
- `ch2b_bad_register.rs`：在U态访问 S 态寄存器。

#### 第二题

1. `trap_handler`返回后`__restore`开始执行，所以此时`a0`代表的值是`trap_handler`的返回值。而返回值是一个`&mut TrapContext`，故此时`a0`中的值是内核栈中上下文的地址。

   - 第一个使用场景：从trap处理结束后，从S态恢复到U态；
   - 第二个使用场景：向用户栈中压入程序的初始上下文。

2. `sstatus`, `sepc`, `sscratch`. 

   - `sstatus`标记CPU处于的特权级信息，实现从S态到U态的切换。
   - `sepc`记录trap发生前最后一条指令的地址，保护现场时对此值+4，使trap返回后继续执行下一条指令。

   - `sscratch`在进入用户态时用于暂存用户栈地址，实现上下文恢复时从内核栈到用户栈的切换。

3. 不保存x2(sp)是因为后续寄存器备份的位置需要通过sp确定。不保存x4(tp)是因为一般不会被使用。

4. sp中的值是用户栈栈顶位置，sscratch中的值是内核栈栈顶位置。

5. ` csrw sstatus, t0`，因为处理器当前所处状态由`sstatus`寄存器标志决定。

6. sp中的值是内核栈栈顶位置，sscratch中的值是用户栈栈顶位置。

7. ` call trap_handler`后，在`trap_handler`函数中调用`syscall`函数，`syscall`中嵌入汇编`ecall`指令。

### 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> [RISC-V 特权级文档](https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf) \
> [Calling Convention](https://en.wikichip.org/wiki/risc-v/registers) \
> 注：以上来源均无直接代码参考

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

