import { invoke } from "@tauri-apps/api/core";
import { Task } from "../../models/Task";
import { Checkbox } from "../Inputs/Checkbox";
import { useState } from "react";
import { TaskDetails } from "./TaskDetails";

interface TaskRowProps {
  task: Task;
  onCheckTask: (task: Task) => void;
}
export const TaskRow = (props: TaskRowProps) => {
  const [detailsVisible, setDetailsVisible] = useState<boolean>(false);
  const onCheckTask = async (checked: boolean, task: Task) => {
    props.onCheckTask({ ...task, completed: checked });
  };
  return (
    <div
      key={props.task.name}
      className="bg-white mx-4 my-1 rounded-xl w-full flex flex-row py-2 px-4 items-center min-h-16"
      onClick={() => {
        setDetailsVisible(true);
        console.log("clicked");
      }}
    >
      <span className="flex flex-row items-center grow-2 just">
        <Checkbox
          checked={props.task.completed}
          onChecked={(e) => {
            onCheckTask(e, props.task);
          }}
        />
        <p className="text-black ml-4 text-xl">{props.task.name}</p>
      </span>
      {props.task.category === "Call" && (
        <span>
          <img
            src="https://api.iconify.design/mingcute:phone-fill.svg?color=%2329506e"
            className={
              " mr-2 bg-hunyadi_yellow-600 hover:bg-hunyadi_yellow-400 rounded-full m-2 p-2 justify-self-end cursor-pointer"
            }
            onClick={(e) => {
              e.stopPropagation();
              console.log("phone clicked");
              invoke("open_dialer", { tel: props.task.tel });
            }}
            alt="update_every"
          />
        </span>
      )}
      {detailsVisible && (
        <TaskDetails
          task={props.task}
          closeDetails={(open: boolean) => setDetailsVisible(open)}
        />
      )}
    </div>
  );
};
