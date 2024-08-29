import React from 'react';
import { Divider } from 'antd';

const ToDoHeader = () => {
  return (
    <div className='flex flex-col items-center justify-center p-2'>
        <h1 className='font-bold text-lg'>To-Do App</h1>
        <Divider style={{ borderColor: "black"}} />
    </div>
  )
}

export default ToDoHeader