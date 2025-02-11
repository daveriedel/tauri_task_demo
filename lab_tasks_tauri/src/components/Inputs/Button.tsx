interface ButtonProps {
  text: string;
  color?: string;
  onClick: () => void;
}

const colorMap = new Map([
  [
    "error",
    "bg-lapis-lazuli-500 hover:bg-lapis-lazuli-400 text-hunyadi_yellow-600",
  ],
  ["success", "bg-green-500"],
]);
export const Button = (props: ButtonProps) => {
  const color: string = props.color
    ? colorMap.get(props.color)
    : "bg-hunyadi_yellow-600 hover:bg-hunyadi_yellow-400 text-lapis-lazuli-400 ";
  return (
    <button
      className={`${color} inset-ring-orange.pantone font-bold py-2 px-4 rounded-xl mr-2`}
      onClick={() => props.onClick()}
    >
      {props.text}
    </button>
  );
};
