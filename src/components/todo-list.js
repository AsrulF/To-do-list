import React from "react";
import { CheckSquareFilled, DeleteFilled } from "@ant-design/icons";
import { Typography } from "antd";

const TodoList = ({ text, id, isComplete, handleDelete, handleDone }) => {
  const { Text } = Typography;
  return (
    <div className="flex justify-between p-2 border rounded-md shadow-lg my-2 bg-white">
      <Text type={isComplete ? "secondary" : null}>{text}</Text>
      <div className="flex justify-between w-14">
        <CheckSquareFilled
          style={{ color: "green" }}
          className="hover:scale-125"
          onClick={() => {
            handleDone(id);
          }}
        />
        <DeleteFilled
          onClick={() => {
            handleDelete(id);
          }}
          style={{ color: "Red" }}
          className="hover:scale-125"
        />
      </div>
    </div>
  );
};

export default TodoList;
