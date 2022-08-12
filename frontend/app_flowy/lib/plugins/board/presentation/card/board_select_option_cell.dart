import 'package:app_flowy/plugins/board/application/card/board_select_option_cell_bloc.dart';
import 'package:app_flowy/plugins/grid/application/cell/cell_service/cell_service.dart';
import 'package:app_flowy/plugins/grid/presentation/widgets/cell/select_option_cell/select_option_cell.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class BoardSelectOptionCell extends StatefulWidget {
  final GridCellControllerBuilder cellControllerBuilder;

  const BoardSelectOptionCell({
    required this.cellControllerBuilder,
    Key? key,
  }) : super(key: key);

  @override
  State<BoardSelectOptionCell> createState() => _BoardSelectOptionCellState();
}

class _BoardSelectOptionCellState extends State<BoardSelectOptionCell> {
  late BoardSelectOptionCellBloc _cellBloc;

  @override
  void initState() {
    final cellController =
        widget.cellControllerBuilder.build() as GridSelectOptionCellController;
    _cellBloc = BoardSelectOptionCellBloc(cellController: cellController)
      ..add(const BoardSelectOptionCellEvent.initial());
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return BlocProvider.value(
      value: _cellBloc,
      child: BlocBuilder<BoardSelectOptionCellBloc, BoardSelectOptionCellState>(
        builder: (context, state) {
          return SelectOptionWrap(
            selectOptions: state.selectedOptions,
            cellControllerBuilder: widget.cellControllerBuilder,
          );
        },
      ),
    );
  }

  @override
  Future<void> dispose() async {
    _cellBloc.close();
    super.dispose();
  }
}
