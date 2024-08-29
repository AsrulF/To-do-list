import logo from './logo.svg';
import './input.css';
import ToDoHeader from './components/header';
import TodoInput from './components/todo-input';
import TodoList from './components/todo-list';


function App() {
  return (
    <div className='w-screen h-screen bg-black flex items-center justify-center'>
      <div className='bg-gray-300 w-[500px] h-[500px] rounded-xl p-3'>
        <ToDoHeader />
        <TodoInput />
      </div>
    </div>
  );
}

export default App;
