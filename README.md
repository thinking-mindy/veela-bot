## Good day human

## Install
- `npm i @thinkingminds/veela`

## Overview
- This is a chatbot based module which can help you create chatbots in web, mobile apps. The chatbot comes with build in responses which are limited and therefore one can add more responses as per requirement. This the free version thus its limited of responses and intelligence but the [Premium] version is comming soon which will be very intelligent and will only be costing from [$69].

- For students who wish to create thier own chatbots, should it be for school assignment or self app, feel free to use this project's free version as a starting point.

## For developers
If you need, by any chance to modify the source code, you so allowed to do so. Also feel free to resuse and redeploy. If you also have any suggestion or a better insight please feel free to contact me on the contact details provided below.

To use this in your node js app, simply, import the [DA] function and simply pass the text you want to be evaluated. The response can be stored in any variable of your choice.

## Example usage

> Node js
```javascript
const da=require('@thinkingminds/veela');
const readline=require('readline').createInterface({input: process.stdin,output: process.stdout});

readline.question('Reply: ',req=>{
    const response=da.DA(req);
	console.log(response)
	readline.close();
	});
```
> React
```javascript
import {DA} from '@thinkingminds/veela'
export default function App(){

	const [res,setRes]=useState();
	
	useEffect(()=>setRes(DA("your_text here")
	return(
	<div>{res}</div>
	)
}
```

## Reporting Issues
If you'd like to report a bug then: contact me as follows:

- Call/ Whatsapp: +263771657582
- Email: talentjahtale@gmail.com

## Licensing

- Copyright 2024 Thinking Minds (https://thinkingminds.co.zw)

- Licensed under Thinking Minds Open-Sourced

## Useful Links

- [The Great Valley](https://thinkingminds.co.zw)
- [Niaxa Technologies](https://niaxatechnologies.co.zw)

##  THINKING MINDS
