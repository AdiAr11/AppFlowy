import 'package:app_flowy/plugins/grid/application/row/row_cache.dart';
import 'package:flutter/material.dart';

class BoardCard extends StatelessWidget {
  final RowInfo rowInfo;

  const BoardCard({required this.rowInfo, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const SizedBox(height: 20, child: Text('1234'));
  }
}
