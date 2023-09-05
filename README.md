# README

## Introduce

`myfind`是一个使用rust编写的命令行查找工具，不依赖系统API，可完成轻量级的文件查找任务。

## 使用说明

本程序已测试支持的命令行环境有：

> windows10: powershell
> 
> linux: Ubuntu 22.04 LTS zsh/bash

**使用方法：**

1. 克隆代码库 `git clone`

2. 在仓库根目录使用cargo build，可以使用--release，使用与否会导致可执行程序生成目录不同

3. [可选]将可执行文件目录添加到环境变量

4. 如未进行3，可以使用相对路径/绝对路径使用程序，如下面截图所示，注意可执行文件所在的子目录随build的参数是否选择--release而变化。分别为`./target/debug`和`./target/release`可执行文件名为myfind，完整基础使用方法如下所示![use1.png](https://img1.imgtp.com/2023/09/05/YpEc9hmN.png)
   
   输入参数目录和文件需要匹配的正则，可选参数`-s/-v/-l`

5. 支持使用绝对路径和相对路径，如果目录中存在空格需要使用`"`包裹

## Features

##### 基础用法如下：可在命令行使用 --help/-h 显示![myfind_help.png](https://img1.imgtp.com/2023/09/05/wbWLnwsC.png)

##### 参数用法

`-s/--sort` ：指定查找结果的排序方式

> 可选值：
> 
> - path：按照与给出目录距离排序，首先输出该文件夹中的内容，在输出子文件夹内容...，这种排序方式是默认的选择。
> 
> - access：按照文件最后访问时间排序，最近使用的文件排在前面，防止名称的干扰文件扰乱查找。

`-v/--verbose`：用更详尽的形式呈现结果，类似ls -l命令

`-l/--limit`：由于目录层级可能较高，符合不是很严格的正则的结果可能有很多，通过`limit`控制显示结果，防止刷屏，可以配合`-s/--sort`更容易和方便的找到想要的文件。

##### 其他实现

- 使用命令行程序实现使用了`clap` `crate`，使用显示美观，包含`--help`辅助使用

- 实现了清晰的错误指示，辅助使用者正确使用程序

- 实现了多彩的命令行

- 使用tracing实现日志，可通过调节代码实现更多日志的打印，默认日志级别较高，不会显示`info`日志

##### 使用展示

注：由于在`linux`和`windows`下结果大同小异，只展示在`linux`下的运行结果

测试目录结构如下

- <img title="" src="https://img1.imgtp.com/2023/09/05/zeXqcVMR.png" alt="testdir.png" data-align="inline">
1. 基础使用
   
   1. 绝对路径
      
      ![abs.png](https://img1.imgtp.com/2023/09/05/jvlkYFss.png)
   
   2. 相对路径
      
      ![rel.png](https://img1.imgtp.com/2023/09/05/XGZYxnfW.png)

2. 使用参数-v/--verbose
   
   1. 使用
      
      - ![verbose.png](https://img1.imgtp.com/2023/09/05/Vsm6oMwN.png)
   
   2. 不使用
      
      ![rel.png](https://img1.imgtp.com/2023/09/05/XGZYxnfW.png)

3. 使用参数-s/--sort
   
   1. 不使用（默认为path）
      
      ![rel.png](https://img1.imgtp.com/2023/09/05/XGZYxnfW.png)
   
   2. 使用参数access，可见按照访问时间排序
      
      ![access.png](https://img1.imgtp.com/2023/09/05/nqF9KH6U.png)

4. 使用参数-l/--limit
   
   1. 不使用（默认为500个），在当前目录查找出现很多干扰
      
      ![notl.png](https://img1.imgtp.com/2023/09/05/ebY4LdUF.png)
   
   2. 使用限制
      
      ![limit.png](https://img1.imgtp.com/2023/09/05/Bjm5qfnJ.png)

5. 使用参数-V/--version
   
   ![version.png](https://img1.imgtp.com/2023/09/05/uMuuy61g.png)

6. 使用参数-h/--help
   
   ![help.png](https://img1.imgtp.com/2023/09/05/6jkFpZ0k.png)

7. 错误反馈
- ![unknownx.png](https://img1.imgtp.com/2023/09/05/cxxY25qD.png)
  
  ![dirfault.png](https://img1.imgtp.com/2023/09/05/h4AdS7Jl.png)

##### 关于未实现功能的说明

在老师提供的可选feature中，没有实现多个正则和多个路径，首先是因为由于命令行程序的特点，如果含有可变数量的参数可能会破环命令行程序的简洁性，多个参数的分隔和传递如果使用类似C命令行参数语法的传递数量和内容，对使用者来说不够简单。

此外，匹配多个正则可是使用正则语法直接实现，还可以使用正则语法实现且或非的关系，将这种细节交予命令行程序，我认为不太必要。多个路径可以简单的通过多次命令的实现做到。

##### 乞讨一下分数

老师提出的feature大多的实现了，对于没有的要求也有实现，其他短学期课程给分都好高好高的，乞求助教给给高分qwq
