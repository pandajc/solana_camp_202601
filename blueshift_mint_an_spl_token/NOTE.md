##### 初始化ts项目
```
mkdir blueshift_mint_an_spl_token
cd blueshift_mint_an_spl_token
npm init -y
npm install -g typescript
tsc --init

npm install [packages]
...
```


##### 要使用 process.env
```
npm i --save-dev @types/node
tsconfig.json 修改
"types": ["node"],
```

###### 解决ESM syntax is not allowed in a CommonJS module when 'verbatimModuleSyntax' is enabled.ts

adding "type": "module" to your package.json
