defmodule EzeWorks do
  use Application

  def start(_type, _args) do
    children = [
      {Bandit, plug: EzeWorks.Router, port: 3000}
    ]

    Supervisor.start_link(children, strategy: one_for_one, name: EzeWorks)
  end
end
