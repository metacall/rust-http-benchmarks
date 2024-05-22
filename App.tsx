import React, { useState, ReactNode } from 'react';
import { renderToString } from 'react-dom/server';

export function Fibonacci(num: number, result: number) {
	return renderToString(
		<div>
			<script src="https://cdn.tailwindcss.com"></script>
			<div className="text-4xl font-bold">F<sub>{num}</sub> = {result} </div>
		</div>
	);
}

export function Hello(name: String) {
	return renderToString(<div>
		<script src="https://cdn.tailwindcss.com"></script>
		<h1 className="text-4xl font-bold">Hello, <span className='capitalize'>{name}</span>!</h1>
	</div>)
}
