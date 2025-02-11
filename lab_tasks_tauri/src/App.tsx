import { useState } from "react";
import "./App.css";
import { AddTaskModal } from "./components/Task/AddTaskModal";
import { TaskPage } from "./pages/task/TaskPage";
import { Fab } from "./components/Inputs/Fab";
import { TaskContext } from "./components/Task/TaskContext";
import { Task } from "./models/Task";

function App() {
  const [showModal, setShowModal] = useState(false);
  const [tasks, setTasks] = useState<Array<Task>>([]);
  return (
    <main className="container">
      <TaskContext.Provider value={{ tasks, setTasks }}>
        <TaskPage />
        {showModal && (
          <AddTaskModal
            onClose={() => {
              console.log("closing modal");
              setShowModal(false);
            }}
            onSave={() => {}}
          />
        )}
        <span className="fixed bottom-8 right-8">
          <Fab
            onClick={() => {
              setShowModal(true);
            }}
          />
        </span>
      </TaskContext.Provider>
    </main>
  );
}

export default App;
