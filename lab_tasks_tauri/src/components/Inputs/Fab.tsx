interface FabProps {
  onClick: () => void;
}
export const Fab = (props: FabProps) => {
  return (
    <button
      className="bg-hunyadi_yellow-600 hover:bg-hunyadi_yellow-400 text-lapis-lazuli-400 rounded-full w-16 h-16 bold text-3xl"
      onClick={() => {
        props.onClick();
      }}
    >
      +
    </button>
  );
};
