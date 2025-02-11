interface CheckboxProps {
  label?: string;
  onChecked: (e: boolean) => void;
  checked: boolean;
}

export const Checkbox = (props: CheckboxProps) => {
  console.log(props.checked);
  return (
    <span className="flex flex-row">
      {props.label && <p>{props.label}:</p>}
      <input
        checked={props.checked}
        type="checkbox"
        onChange={(e) => {
          props.onChecked(e.target.checked);
        }}
      />
    </span>
  );
};
