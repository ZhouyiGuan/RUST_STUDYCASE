####
生成page网站： 

1）cargo doc --no-deps --document-private-items 
（cargo doc --no-deps 不包括依赖项  cargo doc --no-deps --document-private-items 只有自己的模块）

2）在target/doc/文件夹下创建一个index.html,内容如下；其中url等号后面是自己的项目名，也就是src目录下自己模块的名字
    <meta http-equiv="refresh" content="0; url=actix">

3）把target/doc文件夹打包传到项目下一个新分支：page

4）在github中settings/Pages中的Branch下面设置来自page，然后是root目录。

5）save完了之后如果构建成功就会在这个页面生成一个链接