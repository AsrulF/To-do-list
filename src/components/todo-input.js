import { Button, Input, Space } from "antd";
import React, { useEffect, useRef, useState } from "react";
import TodoList from "./todo-list";

const TodoInput = () => {
  const [toDoList, setToDoList] = useState([]);

  const inputRef = useRef(null);

  const handleAdd = () => {
    const inputText = inputRef.current.input.value.trim();

    console.log(inputText);

    if (inputText === "") {
      return alert("Please Input The Task");
    } else {
      const newTask = {
        id: Date.now(),
        text: inputText,
        isComplete: false,
      };

      console.log(newTask);

      setToDoList([...toDoList, newTask]);
    }
  };

  const handleDelete = (id) => {
    const updateTodos = toDoList.filter((todo) => todo.id !== id);
    setToDoList(updateTodos);
  };

  const handleDone = (id) => {
    const doneTodos = toDoList.map((todo) => {
      if (todo.id === id) {
        return { ...todo, isComplete: !todo.isComplete };
      } else {
        return todo;
      }
    });

    setToDoList(doneTodos);
  };

  useEffect(() => {
    console.log(toDoList);
  }, [toDoList]);

  return (
    <div className="flex-col items-center justify-center gap-2">
      <div className="flex gap-2 w-full">
        <Space.Compact style={{ width: "100%" }}>
          <Input
            ref={inputRef}
            size="large"
            placeholder="Type your task here"
            onPressEnter={handleAdd}
          />
          <Button type="primary" onClick={handleAdd} size="large">
            ADD
          </Button>
        </Space.Compact>
      </div>
      {toDoList.map((todo, index) => {
        return (
          <TodoList
            key={index}
            text={todo.text}
            id={todo.id}
            isComplete={todo.isComplete}
            handleDelete={handleDelete}
            handleDone={handleDone}
          />
        );
      })}
    </div>
  );
};

export default TodoInput;
